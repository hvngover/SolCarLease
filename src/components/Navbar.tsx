import React from 'react';
import Link from 'next/link';

const Navbar = () => {
    return (
        <nav className={'navbar'}>
            <div className="container mx-auto px-4 py-8">
                <h1 className="text-3xl font-bold mb-4">SolCarLease dApp</h1>
                <p>Welcome to SolCarLease dApp. Please navigate to:</p>
                <ul className="flex flex-row list-none space-x-4">
                    <li className="px-4 py-2 hover:bg-gray200 rounded-md">
                        <Link href="/" className="text-gray-700 font-medium">
                            Main Page
                        </Link>
                    </li>
                    <li className="px-4 py-2 hover:bg-gray-200 rounded-md">
                        <Link href="/lease" className="text-gray-700 font-medium">
                            Lease a Car
                        </Link>
                    </li>
                    <li className="px-4 py-2 hover:bg-gray-200 rounded-md">
                        <Link href="/payment" className="text-gray-700 font-medium">
                            Make a Payment
                        </Link>
                    </li>
                    <li className="px-4 py-2 hover:bg-gray-200 rounded-md">
                        <Link href="/leaseAgreements" className="text-gray-700 font-medium">
                            View Lease Agreements
                        </Link>
                    </li>
                </ul>
            </div>
        </nav>
    );
};

export default Navbar;
