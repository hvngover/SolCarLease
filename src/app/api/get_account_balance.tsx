// @ts-ignore
import { connectToNetwork, getAccountBalance } from '../../../solcarlease/src/lib.rs';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import type { NextApiRequest, NextApiResponse } from 'next';

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
    const client = await connectToNetwork();
    const { pubkey } = req.query;

    if (!pubkey || Array.isArray(pubkey)) {
        res.status(400).json({ error: 'Invalid public key' });
        return;
    }

    try {
        const publicKey = new PublicKey(pubkey);
        const balance = await getAccountBalance(client, publicKey);
        res.status(200).json({ balance });
    } catch (error) {
        console.error('Error fetching account balance:', error);
        res.status(500).json({ error: 'Internal Server Error' });
    }
}