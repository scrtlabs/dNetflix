<template>
    <div class="main">
        <template v-if="!walletAddress && !isMobile">
            <v-btn @click="connect">Connect Wallet</v-btn>
        </template>
        <template v-else>
            <v-dialog v-model="showQRDialog" max-width="540">
                <div style="display: flex; justify-content: center; align-items: center; flex-direction: column; background-color: black">
                    <div style="font-size: 20px">Scan this QR code with your mobile device</div>
                    <div style="background-color: white; padding: 20px; width: 540px; height: 540px">
                        <qr-code :size="500"  :text="qrData"></qr-code>
                    </div>
                </div>
            </v-dialog>

            <div v-if="!isMobile">
                Address: <span class="wallet">{{ walletAddress }}</span>
            </div>
            <div style="margin-top: 10px">
                <template v-if="balance > 0">
                    <v-btn v-if="NFTs.length === 0 && !isMobile" @click="mint" :disabled="minting">{{ minting ? 'Minting...' : 'Mint' }}</v-btn>
                    <v-btn v-if="NFTs.length > 0 && !isMobile" @click="showQRDialog = true">Open on mobile</v-btn>
                    <template v-if="isMobile && allowToScan">
                        <div style="display: flex; flex-direction: column; align-items: center">
                            <div  v-if="!showQRScanner">Open my collection on your desktop and scan the code</div>
                            <v-btn v-if="!showQRScanner" @click="showQRScanner = true">Scan</v-btn>
                            
                            <div style="margin-bottom: 20px" v-if="showQRScanner && cameraInfo != ''">{{ cameraInfo }}</div>
                            <v-progress-circular v-if="waitingForQRScan && showQRScanner" :size="100" :width="12" color="orange" indeterminate></v-progress-circular>
                            
                            <qrcode-stream v-if="showQRScanner" @decode="onDecode" @init="onInit"></qrcode-stream>
                        </div>
                    </template>
                </template>
                <template v-else>
                    <div>You don't have enough SCRT in your wallet</div>
                </template>

            </div>
            <div>
                <v-container v-if="loadingTokens">
                    <v-row align="center" no-gutters>
                        <v-col>
                            <div class="item-loading">{{ loadingText }}</div>
                        </v-col>
                    </v-row>

                    <v-row align="center" no-gutters style="margin-bottom: 10px">
                        <v-col>
                            <v-progress-linear height="20" striped  color="deep-orange" v-model="progressBarValue" :active="true" :indeterminate="totalTokens < 0" :query="true"></v-progress-linear>
                        </v-col>
                    </v-row>

                    <v-row align="center" no-gutters>
                        <v-col>
                            <v-skeleton-loader class="mx-auto"  type="card" min-width="300" ></v-skeleton-loader>
                        </v-col>
                    </v-row>
                </v-container>
                <v-container>
                    <template v-if="isMobile && !loadingTokens && NFTs.length > 0 && !allowToScan">
                        <div style="text-align: center; width: 100%">
                            <v-btn @click="changeAllowToScan(true)">Scan Again</v-btn>
                        </div>
                    </template>
                    
                    <v-row align="center" no-gutters>
                        <v-col v-if="balance > 0 && !loadingTokens && NFTs.length == 0 ">
                            No items, please mint one
                        </v-col>
                        <v-col v-for="(item, index) in NFTs" :key="'table_item_' + index" class="item" style="margin-right: 5px; margin-left: 5px">
                            <v-card class="mx-auto my-12" width="300" >
                                <a v-if="!(showVideoPlayer && playerIdx == index)" :href="getResource(item.public_img)" target="_"><img :src="getResource(item.public_img)" style="width: 300px" /></a>
                                <div>
                                    <mobile-video-player class="video-player" v-if="showVideoPlayer && playerIdx == index" @close="closeVideoPlayer" :source="selectedVideoSource" :videoKey="selectedVideoKey"></mobile-video-player>
                                </div>
                                <v-card-title> {{ item.description }} [{{ item.id }}] </v-card-title>
                                <div class="buttons">
                                    <v-btn :disabled="!item.video.url || !item.video.key" @click="playVideo(item.video.url, item.video.key, index)">Play Video</v-btn>
                                </div>
                            </v-card>
                        </v-col>
                    </v-row>                    
                </v-container>

            </div>
        </template>
    </div>
</template>

<script>
import { mapGetters } from 'vuex';
import { getPermit, getTokens, mintToken } from "../store/snip721-helper";
var base64 = require('base-64');
import Cookies from 'js-cookie'
import { Wallet, SecretNetworkClient } from "secretjs";

export default {
    name: 'IndexPage',
    mounted() {
        var self = this;

        this.$nextTick(() => {
            const permit = Cookies.get('test-permit')
            if (this.$route.query.p != undefined) {
                this.getNFTs(this.$route.query.p);
            } else if (permit != undefined) {
                this.getNFTs(permit);
            } else {
                this.$store.commit("setAllowToScan", true);
            }

            var connectedBefore = window.localStorage.getItem('connectedBefore');
            if (connectedBefore) {
                this.$store.dispatch('initKeplr');
            }            
            
            this.$nuxt.$on('secretjs-loaded', async () => { 
                self.getNFTs();
                self.checkBalance();
            });

            this.$nuxt.$emit('keystorechange', async () => { 
                self.getNFTs();
                self.checkBalance();
            });
            
        });
    },
    computed: {
        ...mapGetters({
            walletAddress: 'getWalletAddress',
            noKeplr: 'getNoKeplr',
            keplrLoading: 'getKeplrLoading',
            secretjs: 'getSecretJS',
            nftContract: 'getNFTContract',
            chainId: 'getChainId',

            isMobile: "isMobile",

            permit: "getPermit",

            loadingTokens: "isLoadingTokens",
            totalTokens: "getTotalTokens",
            loadedTokens: "getLoadedTokens",            
            NFTs: "getCollection",
            allowToScan: "getAllowToScan"

        }),
        progressBarValue() {
            if (this.totalTokens <= 0) {
                return 0;
            }
            return ( (this.loadedTokens) / this.totalTokens) * 100;
        },
        loadingText() {
            if (this.totalTokens < 0) {
                return "Looking for your items...";
            } else {
                return `Loading ${Math.min(this.totalTokens, this.loadedTokens+1) } / ${this.totalTokens}...`;
            }
        }, 
        qrData() {

            let wPermit = {
                address: this.walletAddress,
                data: this.permit
            };

            let p = base64.encode(JSON.stringify(wPermit));

            let url = window.location.protocol + "//" + window.location.hostname;
            if (window.location.port != '' && window.location.port != 443 && window.location.port != 80) {
                url += ":" + window.location.port;
            }
            url += "?p=" + encodeURIComponent(p);
            
            return url;
        },               
    },
    data() {
        return {
            showQRDialog: false, 


            playerIdx: -1,
            showVideoPlayer: false,
            selectedVideoSource: "",
            selectedVideoKey: "",

            showQRScanner: false,
            waitingForQRScan: true,
            cameraInfo: "Waiting for camera...",

            minting: false,

            balance: 0


        }
    },
    methods: {
        async checkBalance() {
            var answer = await this.secretjs.query.bank.balance(
            {
                address: this.walletAddress,
                denom: "uscrt",
            });
            this.balance = answer.balance.amount;
        },
        getResource(resource) {
            resource = resource.replace("ipfs://", "");
            return `${process.env.NUXT_ENV_IPFS_GATEWAY_URL}/${resource}`;
        },
        playVideo(video, key, index) {
            this.selectedVideoSource = this.getResource(video);
            this.selectedVideoKey = key;
            this.showVideoPlayer = true;
            this.playerIdx = index;
        },        
        closeVideoPlayer() {
            this.showVideoPlayer = false;
            this.playerIdx = -1;
        },        
        connect() {
            this.$store.dispatch('initKeplr');
        },
        changeAllowToScan(value) {
            this.$store.commit("setAllowToScan", value);
        },
        async mint() {
            this.minting = true;
            let result = await mintToken(this.secretjs, this.walletAddress, this.nftContract);
            if (result) {
                this.getNFTs();
            }
            this.minting = false;
        },

        async onInit (promise) {
            console.log("on Init");
            this.waitingForQRScan = true;            
            this.cameraInfo = "Waiting for camera...";
            try {
                const { capabilities } = await promise

            // successfully initialized
            } catch (error) {
                console.log(error);
                if (error.name === 'NotAllowedError') {
                    this.cameraInfo = "Please allow to use the camera";
                } else if (error.name === 'NotFoundError') {
                    this.cameraInfo = "Cannot detect camera";
                } else if (error.name === 'NotSupportedError') {
                    this.cameraInfo = "Cannot open camera under non-secure HTTP connection";
                } else if (error.name === 'NotReadableError') {
                    this.cameraInfo = "Cannot open camera";
                } else if (error.name === 'OverconstrainedError') {
                    this.cameraInfo = "Cannot open camera";
                } else if (error.name === 'StreamApiNotSupportedError') {
                    this.cameraInfo = "Cannot open camera";
                }
                this.waitingForQRScan = false;
            } finally {
                this.waitingForQRScan = false;
                this.cameraInfo = "";
            }
        },
        
        onDecode(data) {
            try {
                if (data.indexOf("?p=") > 0) {
                    let tmp = data.split('=');
                    this.getNFTs(tmp[1]);
                }
            } catch (err) {
                console.log(err);
            }
            this.showQRScanner = false;
        },        
        
        async getNFTs(strPermit) {
            this.$store.dispatch('getCollection', strPermit);
        }
    }
};
</script>

<style scoped>
.main {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.wallet {
    color: orange;
}

.buttons {
    margin-top: 30px;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
}

.buttons button {
    margin: 10px 0px 10px 0px;
    width: 80%;
    height: 60px;
    background-color: #29120c;
    color: rgba(224, 86, 55, 0.8);
    font-style: normal;
    font-weight: 400;
    font-size: 16px;
    line-height: 20px;
}
</style>
