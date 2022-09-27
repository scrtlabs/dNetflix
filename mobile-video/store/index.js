import { getVideoCollectionPermit, getVideoTokens } from '../store/snip721-helper';
import { SecretNetworkClient } from 'secretjs';

export const state = () => ({
    secretjs: null,

    rest: process.env.NUXT_ENV_REST_URL,
    rpcUrl: process.env.NUXT_ENV_RPC_URL,
    grpcWebUrl: process.env.NUXT_ENV_GRPCWEB_URL,
    chainId: process.env.NUXT_ENV_CHAIN_ID,

    keplrLoading: false,
    noKeplr: false,
    walletAddress: '',

    accessManagerContract: {
        address: process.env.NUXT_ENV_CONTRACT,
        codeHash: process.env.NUXT_ENV_CONTRACT_CODE_HASH
    },

    loadingTokens: false,
    totalTokens: -1,
    loadedTokens: 0,
    videoCollection: []
});

export const mutations = {
    setSecretJS(state, obj) {
        state.secretjs = obj;

        setTimeout(() => {
            $nuxt.$emit('secretjs-loaded');
        }, 1000);
    },
    setKeplrLoading(state, loading) {
        state.keplrLoading = loading;
    },
    setNoKeplr(state, value) {
        state.noKeplr = value;
    },
    setWalletAddress(state, address) {
        state.walletAddress = address;
    },
    setIsLoadingTokens(state, value) {
        state.loadingTokens = value;
    },
    setTotalTokens(state, value) {
        state.totalTokens = value;
    },
    setLoadedTokens(state, value) {
        state.loadedTokens = value;
    },
    setVideoCollection(state, value) {
        state.videoCollection = value;
    }
};

export const actions = {
    initKeplr({ commit, state }) {
        const keplrConnect = async (addEvent) => {
            addEvent = typeof addEvent == 'undefined' ? true : addEvent;

            // Add indication for UI when keplr is loading
            commit('setKeplrLoading', true);

            // Check if the browser has keplr wallet extension
            if (!window.keplr || !window.getEnigmaUtils || !window.getOfflineSignerOnlyAmino) {
                console.log('Cannot find Keplr wallet');
                commit('setKeplrLoading', false);
                commit('setNoKeplr', true);
            } else {
                var suggestChain = false;

                // Try to activate keplr with the specified chain id.
                // If we fail, we need to suggest the user to add the chain to his wallet
                try {
                    await window.keplr.enable(state.chainId);
                } catch (err) {
                    if (err.message.indexOf('no chain info') != -1) {
                        suggestChain = true;
                    } else {
                        return;
                    }
                }

                try {
                    if (suggestChain) {
                        await window.keplr.experimentalSuggestChain({
                            rpc: state.rpcUrl,
                            rest: state.rest,
                            chainId: state.chainId,
                            chainName: state.chainId,
                            stakeCurrency: {
                                coinDenom: 'SCRT',
                                coinMinimalDenom: 'uscrt',
                                coinDecimals: 6,
                                coinGeckoId: 'secret'
                            },
                            bip44: {
                                coinType: 529
                            },
                            bech32Config: {
                                bech32PrefixAccAddr: 'secret',
                                bech32PrefixAccPub: 'secret' + 'pub',
                                bech32PrefixValAddr: 'secret' + 'valoper',
                                bech32PrefixValPub: 'secret' + 'valoperpub',
                                bech32PrefixConsAddr: 'secret' + 'valcons',
                                bech32PrefixConsPub: 'secret' + 'valconspub'
                            },
                            currencies: [
                                {
                                    coinDenom: 'SCRT',
                                    coinMinimalDenom: 'uscrt',
                                    coinDecimals: 6,
                                    coinGeckoId: 'secret'
                                }
                            ],
                            feeCurrencies: [
                                {
                                    coinDenom: 'SCRT',
                                    coinMinimalDenom: 'uscrt',
                                    coinDecimals: 6,
                                    coinGeckoId: 'secret'
                                }
                            ],
                            gasPriceStep: { low: 0.1, average: 0.25, high: 0.3 },
                            features: ['secretwasm']
                        });
                    }

                    const keplrOfflineSigner = window.getOfflineSignerOnlyAmino(state.chainId);
                    const signer = await keplrOfflineSigner.getAccounts();
                    commit('setWalletAddress', signer[0].address);

                    var secretJS = await SecretNetworkClient.create({
                        grpcWebUrl: state.grpcWebUrl,
                        chainId: state.chainId,
                        wallet: keplrOfflineSigner,
                        walletAddress: state.walletAddress,
                        encryptionUtils: window.getEnigmaUtils(state.chainId)
                    });
                    commit('setSecretJS', secretJS);

                    // Save an indication that the user already approved keplr so we won't
                    // Pop-up a keplr window before he pressed the "connect wallet" at least one time
                    window.localStorage.setItem('connectedBefore', '1');

                    // Listen to keplr events when wallet change
                    if (addEvent) {
                        window.addEventListener('keplr_keystorechange', () => {
                            keplrConnect(false);
                            $nuxt.$emit('keystorechange');
                        });
                    }
                } catch (err) {
                    console.log(err.message);
                    console.log('Cannot connect to your wallet<br>Please make sure Keplr is installed properly');
                }
                commit('setKeplrLoading', false);
            }
        };
        keplrConnect();
        //setTimeout(keplrConnect, 1000);
    },

    async getVideoCollection({ commit, state }) {
        commit('setVideoCollection', []);
        commit('setIsLoadingTokens', true);
        commit('setTotalTokens', -1);
        commit('setLoadedTokens', 0);

        const msg = { list_videos: { page: 0, page_size: 100 } };
        var activeSecretjs = state.secretjs;

        let localCollection = [];

        let list = await activeSecretjs.query.compute.queryContract({
            contractAddress: state.accessManagerContract.address,
            codeHash: state.accessManagerContract.codeHash,
            query: msg
        });

        let videoList = list.list_videos.videos;
        commit('setTotalTokens', videoList.length);

        console.log(JSON.stringify(videoList));

        let NFTList = []; // To create one permit for all the collections, avoiding Keplr pop-up on every permit request
        for (let i = 0; i < videoList.length; i++) {
            console.log(videoList[i]);
            localCollection.push({
                id: videoList[i].id,
                name: videoList[i].name,
                cover: videoList[i].image,
                price: (videoList[i].price.amount / 1000000).toFixed(2),
                token: {
                    address: videoList[i].access_token.address,
                    codeHash: videoList[i].access_token.hash
                }
            });
            NFTList.push(videoList[i].access_token.address);
        }

        let videoPermit = await getVideoCollectionPermit(activeSecretjs, state.walletAddress, NFTList, state.chainId);

        for (var i = 0; i < localCollection.length; i++) {
            try {
                let tokens = await getVideoTokens(activeSecretjs, state.walletAddress, localCollection[i].token.address, videoPermit);
                localCollection[i].purchesed = tokens.length > 0;

                if (localCollection[i].purchesed) {
                    const msg = {
                        with_permit: {
                            query: {
                                nft_dossier: {
                                    token_id: tokens[0]
                                }
                            },
                            permit: videoPermit
                        }
                    };
                    let singleToken = await activeSecretjs.query.compute.queryContract({
                        contractAddress: localCollection[i].token.address,
                        codeHash: localCollection[i].token.address.codeHash,
                        query: msg
                    });

                    var media = singleToken.nft_dossier.private_metadata.extension?.media;
                    if (media) {
                        for (let j = 0; j < media.length; j++) {
                            if (media[j].file_type === 'video' && media[j].extension === 'mp4') {
                                localCollection[i].video_url = media[j].url;
                                localCollection[i].video_key = media[j].authentication.key;
                                break; // Take only the 1st for the demo
                            }
                        }
                    }
                }
            } catch (err) {
                console.log(err);
            }
            commit('setLoadedTokens', state.loadedTokens + 1);
        }
        commit('setVideoCollection', localCollection);
        commit('setIsLoadingTokens', false);
    }
};

var mobileAndTabletCheck = function () {
    let check = false;
    (function (a) {
        if (
            /(android|bb\d+|meego).+mobile|avantgo|bada\/|blackberry|blazer|compal|elaine|fennec|hiptop|iemobile|ip(hone|od)|iris|kindle|lge |maemo|midp|mmp|mobile.+firefox|netfront|opera m(ob|in)i|palm( os)?|phone|p(ixi|re)\/|plucker|pocket|psp|series(4|6)0|symbian|treo|up\.(browser|link)|vodafone|wap|windows ce|xda|xiino|android|ipad|playbook|silk/i.test(
                a
            ) ||
            /1207|6310|6590|3gso|4thp|50[1-6]i|770s|802s|a wa|abac|ac(er|oo|s\-)|ai(ko|rn)|al(av|ca|co)|amoi|an(ex|ny|yw)|aptu|ar(ch|go)|as(te|us)|attw|au(di|\-m|r |s )|avan|be(ck|ll|nq)|bi(lb|rd)|bl(ac|az)|br(e|v)w|bumb|bw\-(n|u)|c55\/|capi|ccwa|cdm\-|cell|chtm|cldc|cmd\-|co(mp|nd)|craw|da(it|ll|ng)|dbte|dc\-s|devi|dica|dmob|do(c|p)o|ds(12|\-d)|el(49|ai)|em(l2|ul)|er(ic|k0)|esl8|ez([4-7]0|os|wa|ze)|fetc|fly(\-|_)|g1 u|g560|gene|gf\-5|g\-mo|go(\.w|od)|gr(ad|un)|haie|hcit|hd\-(m|p|t)|hei\-|hi(pt|ta)|hp( i|ip)|hs\-c|ht(c(\-| |_|a|g|p|s|t)|tp)|hu(aw|tc)|i\-(20|go|ma)|i230|iac( |\-|\/)|ibro|idea|ig01|ikom|im1k|inno|ipaq|iris|ja(t|v)a|jbro|jemu|jigs|kddi|keji|kgt( |\/)|klon|kpt |kwc\-|kyo(c|k)|le(no|xi)|lg( g|\/(k|l|u)|50|54|\-[a-w])|libw|lynx|m1\-w|m3ga|m50\/|ma(te|ui|xo)|mc(01|21|ca)|m\-cr|me(rc|ri)|mi(o8|oa|ts)|mmef|mo(01|02|bi|de|do|t(\-| |o|v)|zz)|mt(50|p1|v )|mwbp|mywa|n10[0-2]|n20[2-3]|n30(0|2)|n50(0|2|5)|n7(0(0|1)|10)|ne((c|m)\-|on|tf|wf|wg|wt)|nok(6|i)|nzph|o2im|op(ti|wv)|oran|owg1|p800|pan(a|d|t)|pdxg|pg(13|\-([1-8]|c))|phil|pire|pl(ay|uc)|pn\-2|po(ck|rt|se)|prox|psio|pt\-g|qa\-a|qc(07|12|21|32|60|\-[2-7]|i\-)|qtek|r380|r600|raks|rim9|ro(ve|zo)|s55\/|sa(ge|ma|mm|ms|ny|va)|sc(01|h\-|oo|p\-)|sdk\/|se(c(\-|0|1)|47|mc|nd|ri)|sgh\-|shar|sie(\-|m)|sk\-0|sl(45|id)|sm(al|ar|b3|it|t5)|so(ft|ny)|sp(01|h\-|v\-|v )|sy(01|mb)|t2(18|50)|t6(00|10|18)|ta(gt|lk)|tcl\-|tdg\-|tel(i|m)|tim\-|t\-mo|to(pl|sh)|ts(70|m\-|m3|m5)|tx\-9|up(\.b|g1|si)|utst|v400|v750|veri|vi(rg|te)|vk(40|5[0-3]|\-v)|vm40|voda|vulc|vx(52|53|60|61|70|80|81|83|85|98)|w3c(\-| )|webc|whit|wi(g |nc|nw)|wmlb|wonu|x700|yas\-|your|zeto|zte\-/i.test(
                a.substr(0, 4)
            )
        )
            check = true;
    })(navigator.userAgent || navigator.vendor || window.opera);
    return check;
};

export const getters = {
    isMobile(state) {
        var isMobile = mobileAndTabletCheck();
        return isMobile;
    },
    getWalletAddress: (state) => {
        return state.walletAddress;
    },
    getNoKeplr(state) {
        return state.noKeplr;
    },

    getKeplrLoading(state) {
        return state.keplrLoading;
    },
    getSecretJS(state) {
        return state.secretjs;
    },
    getChainId(state) {
        return state.chainId;
    },
    isLoadingTokens(state) {
        return state.loadingTokens;
    },
    getTotalTokens(state) {
        return state.totalTokens;
    },
    getLoadedTokens(state) {
        return state.loadedTokens;
    },
    getVideoCollection(state) {
        return state.videoCollection;
    }
};
