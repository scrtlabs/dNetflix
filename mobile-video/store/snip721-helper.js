export const getVideoCollectionPermit = async (secretJS, walletAddress, contractList, chainId) => {
    if (!secretJS) return null;

    var permKey = `perm_${chainId}_${contractList.join('_')}_${walletAddress}`;
    var permit = null;
    try {
        // Try to load permit from local storage to avoid keplr confirmation in every visit
        permit = JSON.parse(window.localStorage.getItem(permKey));
    } catch (err) {}

    if (!permit) {
        console.log('Loading new permit');
        try {
            const result = await secretJS.utils.accessControl.permit.sign(walletAddress, chainId, 'scrtFlix-permit', contractList, ['balance', 'owner']);
            permit = result;

            // Store the new permit in the browser local storage
            window.localStorage.setItem(permKey, JSON.stringify(permit));
            console.log('Permit loaded');
        } catch (err) {
            console.log('Cannot load permit');
            console.error(err);
        }
    } else {
        console.log('Loading saved permit');
    }
    return permit;
};

export const getVideoTokens = async (secretJS, walletAddress, contractAddress, permit) => {
    if (!secretJS) return [];

    let tokens = [];
    try {
        tokens = await secretJS.query.compute.queryContract({
            contractAddress,
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
