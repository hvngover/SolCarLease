// @ts-ignore
import { connectToNetwork, getAccountBalance } from './/solcarlease/src/lib.rs';
import { Connection, PublicKey, clusterApiUrl } from '@solcarlease/web3.js'

export default async function handler(req, res) {
    const client = connectToNetwork();
    const publicKey = req.query.pubkey;

    try {
        // @ts-ignore
        const balance = await getAccountBalance(client, new Pubkey(publicKey));
        res.status(200).json({ balance });
    } catch (error) {
        // @ts-ignore
        res.status(500).json({ error: error.message });
    }
}