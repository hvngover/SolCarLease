import React from 'react';
import Link from 'next/link';
import './globals.css';
import Navbar from "@/Navbar/Navbar";
import '@solana/web3.js'

export default function Home() {
    return (
        <div>
            <Navbar/>
        </div>
    );
}