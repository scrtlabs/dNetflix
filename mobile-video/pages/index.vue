<template>
    <div class="main">
        <template v-if="!walletAddress">
            <v-btn @click="connect">Connect Wallet</v-btn>
        </template>
        <template v-else>
            <div>
                Address: <span class="wallet">{{ walletAddress }}</span>
            </div>
            <div style="text-align: center">
                <v-btn @click="showAdd = !showAdd">{{ showAdd ? 'Close Video Adding Panel' : 'Add Video' }}</v-btn>

                <v-expand-transition>
                    <v-card width="500" style="margin-top: 10px" v-show="showAdd">
                        <v-card-text>
                            <v-text-field label="Video Name" v-model="newVideo.name"></v-text-field>
                            <v-text-field label="Cover image URL" v-model="newVideo.image_url"></v-text-field>
                            <v-text-field label="Video URL" v-model="newVideo.video_url"></v-text-field>
                            <v-text-field label="Decryption Key" v-model="newVideo.decryption_key"></v-text-field>
                            <div>
                                <v-text-field label="Price" v-model="newVideo.price" suffix="$SCRT" type="number"></v-text-field>
                                <v-slider v-model="newVideo.royalty" max="50" min="0" label="Royalty" thumb-color="green">
                                    <template v-slot:thumb-label="{ value }"> {{ value }}% </template>
                                </v-slider>
                            </div>
                        </v-card-text>
                        <v-card-actions>
                            <v-btn :disabled="adding" color="orange" text @click="addVideo"> {{ adding ? 'Wait...' : 'Add' }} </v-btn>
                            <v-btn :disabled="adding" color="orange" text @click="clearForm"> Clear </v-btn>
                        </v-card-actions>
                    </v-card>
                </v-expand-transition>
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
                            <v-progress-linear
                                height="20"
                                striped
                                color="deep-orange"
                                v-model="progressBarValue"
                                :active="true"
                                :indeterminate="totalTokens < 0"
                                :query="true"
                            ></v-progress-linear>
                        </v-col>
                    </v-row>

                    <v-row align="center" no-gutters>
                        <v-col>
                            <v-skeleton-loader class="mx-auto" type="card" min-width="300"></v-skeleton-loader>
                        </v-col>
                    </v-row>
                </v-container>
                <v-container>
                    <v-row align="center" no-gutters>
                        <v-col v-if="!loadingTokens && Videos.length == 0"> No videos available</v-col>
                        <v-col v-for="(item, index) in Videos" :key="'table_item_' + index" class="item" style="margin-right: 5px; margin-left: 5px">
                            <v-card class="mx-auto my-12" width="300">
                                <a v-if="!(showVideoPlayer && playerIdx == index)" :href="getResource(item.cover)" target="_"
                                    ><img :src="getResource(item.cover)" style="width: 300px"
                                /></a>
                                <div>
                                    <mobile-video-player
                                        class="video-player"
                                        v-if="showVideoPlayer && playerIdx == index"
                                        @close="closeVideoPlayer"
                                        :source="selectedVideoSource"
                                        :videoKey="selectedVideoKey"
                                    ></mobile-video-player>
                                </div>
                                <v-card-title>Title: {{ item.name.toUpperCase() }}</v-card-title>
                                <v-card-subtitle>{{ item.price }} SCRT</v-card-subtitle>
                                <v-card-actions>
                                    <v-btn v-if="item.purchesed === false" color="orange" :disabled="buying != -1" text @click="buy(item.id, item.price)">{{ buying == item.id ? 'Wait...' : 'Buy' }}</v-btn>
                                    <v-btn v-if="item.purchesed === true" color="orange" :disabled="!item.video_url || !item.video_key" @click="playVideo(item.video_url, item.video_key, index)">Play Video</v-btn>
                                </v-card-actions>
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

export default {
    name: 'IndexPage',
    data() {
        return {
            playerIdx: -1,
            showVideoPlayer: false,
            selectedVideoSource: '',
            selectedVideoKey: '',

            minting: false,

            showAdd: false,
            buying: -1, 
            adding: false,

            newVideo: { // Dummy data for new video upload, it can be blank
                name: '',
                price: 1,
                royalty: 15,
                image_url: 'ipfs://QmaK5Y969GeFqcBmu5BAPWgXfwkU9hpQCYJRJyQdYtCBjz',
                video_url: 'ipfs://QmVbKFQRNx166RuqyyEb8XMwnw7GY57g7sXgcmxxKrL9ms/main.m3u8',
                decryption_key: 'UainRqKrHz_62Gfx0Qv4Hg'
            }
        };
    },
    mounted() {
        var self = this;

        this.$nextTick(() => {
            var connectedBefore = window.localStorage.getItem('connectedBefore');
            if (connectedBefore) {
                this.$store.dispatch('initKeplr');
            }

            this.$nuxt.$on('secretjs-loaded', async () => {
                self.listVideos();
            });

            // When changing account or cain in keplr
            this.$nuxt.$emit('keystorechange', async () => {
                self.listVideos();
            });
        });
    },
    computed: {
        ...mapGetters({
            walletAddress: 'getWalletAddress',
            noKeplr: 'getNoKeplr',
            keplrLoading: 'getKeplrLoading',
            secretjs: 'getSecretJS',

            loadingTokens: 'isLoadingTokens',
            totalTokens: 'getTotalTokens',
            loadedTokens: 'getLoadedTokens',
            Videos: 'getVideoCollection',
        }),
        progressBarValue() {
            if (this.totalTokens <= 0) {
                return 0;
            }
            return (this.loadedTokens / this.totalTokens) * 100;
        },
        loadingText() {
            if (this.totalTokens < 0) {
                return 'Looking for your items...';
            } else {
                return `Loading ${Math.min(this.totalTokens, this.loadedTokens + 1)} / ${this.totalTokens}...`;
            }
        },
    },

    methods: {
        getResource(resource) {
            resource = resource.replace('ipfs://', '');
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
        async listVideos() {
            this.$store.dispatch('getVideoCollection');
        },

        async addVideo() {
            this.adding = true;
            let scrtPrice = parseInt(this.newVideo.price * 1000000).toString();
            try {
                let res = await this.secretjs.tx.compute.executeContract(
                    {
                        contractAddress: process.env.NUXT_ENV_CONTRACT,
                        msg: {
                            new_video: {
                                name: this.newVideo.name,
                                royalty_info: {
                                    decimal_places_in_rates: 2,
                                    royalties: [{ recipient: this.walletAddress, rate: parseInt(this.newVideo.royalty) }]
                                },
                                image_url: 'ipfs://QmaK5Y969GeFqcBmu5BAPWgXfwkU9hpQCYJRJyQdYtCBjz',
                                video_url: 'ipfs://QmVbKFQRNx166RuqyyEb8XMwnw7GY57g7sXgcmxxKrL9ms/main.m3u8',
                                decryption_key: 'UainRqKrHz_62Gfx0Qv4Hg',
                                price: { token: { native: 'uscrt' }, amount: scrtPrice }
                            }
                        },
                        sender: this.walletAddress
                    },
                    { gasLimit: 500_000 }
                );
                this.listVideos();
                console.log(res);
            } catch (err) {
                console.log(err);
            }
            this.adding = false;            
        },

        clearForm() {
            this.newVideo = {
                name: '',
                price: 1,
                royalty: 15,
                image_url: '',
                video_url: '',
                decryption_key: ''
            };
        },

        async buy(id, price) {
            this.buying = id;
            let scrtPrice = parseInt(parseFloat(price) * 1000000).toString();
            let res = await this.secretjs.tx.compute.executeContract(
                {
                contractAddress: process.env.NUXT_ENV_CONTRACT,
                msg: {
                    purchase_video: {
                        video_id: id,
                    },
                },
                sender: this.walletAddress,
                sentFunds: [{ denom: "uscrt", amount: scrtPrice }],
                },
                { gasLimit: 500_000 }
            );
            res = res.arrayLog.find((l) => l.key === "minted");
            this.buying = -1;
            this.listVideos();

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
