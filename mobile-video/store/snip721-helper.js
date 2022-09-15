export const getPermit = async (secretJS, walletAddress, contract, chainId) => {
    if (!secretJS)
        return null;

    var permKey = `perm_${chainId}_${contract.address}_${walletAddress}`;
    var permit = null;
    try {
        // Try to load permit from local storage to avoid keplr confirmation in every visit
        permit = JSON.parse(window.localStorage.getItem(permKey));
    } catch (err) {}

    if (!permit) {
        console.log('Loading new permit');
        try {
            const result = await secretJS.utils.accessControl.permit.sign(
                walletAddress,
                chainId,
                "sample-scrt-nft-permit",
                [contract.address],
                ["balance", "owner"]                   
            );
            permit = result;
            
            // Store the new permit in the browser local storage
            window.localStorage.setItem(permKey, JSON.stringify(permit));
            console.log('Permit loaded');
        } catch (err) {
            console.log("Cannot load permit");
            console.error(err);
        }
    } else {
        console.log('Loading saved permit');
    }
    return permit;
};

export const getTokens = async (secretJS, walletAddress, contract, permit) => {
    if (!secretJS)
        return [];

    let tokens = [];
    try {
        tokens = await secretJS.query.compute.queryContract({
            contractAddress: contract.address,
            codeHash: contract.codeHash,
            query: {
                with_permit: {
                    query: {
                        tokens: {
                            owner: walletAddress,
                            limit: 9999
                        }
                    },
                    permit: permit
                }
            }
        });
        return tokens.token_list.tokens;
    } catch (e) {
        console.log(e);
        return [];
    }
};

export const mintToken = async (secretJS, walletAddress, contract) => {
    const publicMetadata = {
        extension: {
            image: "ipfs://QmaK5Y969GeFqcBmu5BAPWgXfwkU9hpQCYJRJyQdYtCBjz",
            description: "Encoded video example",
            attributes: [
                {
                    trait_type: "Animal",
                    value: "Bunny",
                },
            ],
        },
    };

    const privateMetadata = {
        extension: {
            image: "ipfs://QmaK5Y969GeFqcBmu5BAPWgXfwkU9hpQCYJRJyQdYtCBjz",
            description: "Encoded video example",
            attributes: [
                {
                    trait_type: "Animal",
                    value: "Big Bunny",
                },
            ],
            media: [
                {
                    file_type: "video",
                    extension: "m3u8",
                    authentication: {
                        key: "UainRqKrHz_62Gfx0Qv4Hg",
                        user: null,
                    },
                    url: "ipfs://QmVbKFQRNx166RuqyyEb8XMwnw7GY57g7sXgcmxxKrL9ms/main.m3u8",
                },
            ],
        },
    };

    const mintMsg = {
        mint_nft: {
            owner: walletAddress,
            public_metadata: publicMetadata,
            private_metadata: privateMetadata,
        },
    };

    try {
        let tx = await secretJS.tx.compute.executeContract(
            {
                sender: walletAddress,
                contractAddress: contract.address,
                codeHash: contract.codeHash,
                msg: mintMsg,
                sentFunds: [],
            },
            {
                gasLimit: 60_000,
            }
        );
        console.log(tx);
        return true;
    } catch (err) {
        console.error(err);
        return false;
    }

    
}