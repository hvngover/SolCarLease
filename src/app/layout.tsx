import './globals.css';
import { WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import { useMemo } from 'react';
import React from 'react';

require('@solana/wallet-adapter-react-ui/styles.css');

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const wallets = useMemo(() => [new PhantomWalletAdapter()], []);

    return (
        <html lang="en">
            <body>
                <WalletProvider wallets={wallets} autoConnect>
                    <WalletModalProvider>
                        {children}
                    </WalletModalProvider>
                </WalletProvider>
            </body>
        </html>
    );
}
