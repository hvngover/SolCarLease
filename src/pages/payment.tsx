import React, { useState } from 'react';
import axios from 'axios';
import '../app/globals.css'
import Navbar from "@/Navbar/Navbar";

export default function Payment() {
    const [leaseId, setLeaseId] = useState('');
    const [amount, setAmount] = useState('');

    const handlePayment = async () => {
        try {
            const response = await axios.post('/api/payment', { leaseId, amount });
            console.log('Payment response:', response.data);
        } catch (error) {
            console.error('Error making payment:', error);
        }
    };

    return (
        <div>
            <Navbar/>
            <div className="container mx-auto px-4 py-8">
                <h1 className="text-3xl font-bold mb-4">Make a Payment</h1>
                <div className="flex flex-col space-y-2">
                    <input
                        type="text"
                        placeholder="Lease ID"
                        value={leaseId}
                        onChange={(e) => setLeaseId(e.target.value)}
                        className="rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:border-blue-500"
                    />
                    <input
                        type="number"
                        placeholder="Amount"
                        value={amount}
                        onChange={(e) => setAmount(e.target.value)}
                        className="rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:border-blue-500"
                    />
                </div>

                <div className="">
                    <button
                        onClick={handlePayment}
                        className="rounded-md bg-blue-500 text-white px-4 py-2 hover:bg-blue-700 focus:outline-none"
                    >
                        Make Payment
                    </button>
                </div>
            </div>
        </div>
    );
}
