import React, { useState } from 'react';
import axios from 'axios';
import '../app/globals.css';
import Navbar from "@/Navbar/Navbar";

export default function LeaseAgreements() {
    const [leaseId, setLeaseId] = useState('');
    const [agreements, setAgreements] = useState([]);

    const handleFetchLeaseAgreements = async () => {
        try {
            const response = await axios.get(`/lease-agreements?leaseId=${leaseId}`);
            console.log('Lease agreements:', response.data);
            setAgreements(response.data);
        } catch (error) {
            console.error('Error fetching lease agreements:', error);
        }
    };

    return (
        <div>
            <Navbar/>
            <div className="container mx-auto px-4 py-8">
                <h1 className="text-3xl font-bold mb-4">Lease Agreements</h1>
                <div className="flex flex-row space-x-2">  {/* Flexbox container for lease ID and button */}
                    <input
                        type="text"
                        placeholder="Lease ID"
                        value={leaseId}
                        onChange={(e) => setLeaseId(e.target.value)}
                        className="rounded-md border border-gray-300 px-3 py-2 focus:outline-none focus:border-blue-500"
                    />
                    <button
                        onClick={handleFetchLeaseAgreements}
                        className="rounded-md bg-blue-500 text-white px-4 py-2 hover:bg-blue-700 focus:outline-none"
                    >
                        Fetch Lease Agreements
                    </button>
                </div>
                <ul className="mt-4 list-none"> {/* Spacing and removing default list styles */}
                    {agreements.map((agreement, index) => (
                        <li key={index} className="rounded-md p-2 border border-gray-300 hover:bg-gray-200"> {/* Styling for list items */}
                            {agreement}
                        </li>
                    ))}
                </ul>
            </div>
        </div>
    );
}
