import React from 'react';
import Navbar from "@/components/Navbar";
import { useWallet } from '@solana/wallet-adapter-react';
import "./globals.css";

const Home = ({ balance }: { balance: number | null }) => {
    const { publicKey, connect, disconnect } = useWallet();

    const handleLogin = async () => {
        try {
            await connect();
        } catch (error) {
            console.error('Error connecting wallet:', error);
        }
    };

    const handleLogout = async () => {
        try {
            await disconnect();
        } catch (error) {
            console.error('Error disconnecting wallet:', error);
        }
    };

    return (
        <div>
            <Navbar />
            <div className="container mx-auto p-5">
                <h1 className="text-3xl font-bold mb-4">Welcome to SolCarLease dApp!</h1>
                <div className="flex flex-col items-center space-y-4">
                    {publicKey ? (
                        <>
                            <p className="text-lg">Logged in as: {publicKey.toString()}</p>
                            <button className="bg-red-500 text-white px-4 py-2 rounded-md" onClick={handleLogout}>Logout</button>
                        </>
                    ) : (
                        <button className="bg-blue-500 text-white px-4 py-2 rounded-md" onClick={handleLogin}>Login with Phantom</button>
                    )}
                    {balance !== null && (
                        <p>Your account balance: {balance}</p>
                    )}
                </div>
            </div>
        </div>
    );
}

export default Home;
