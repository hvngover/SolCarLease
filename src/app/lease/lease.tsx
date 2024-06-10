import React, { useState } from 'react';
import axios from 'axios';
import Navbar from "@/components/Navbar";
import '../globals.css';

export default function Lease() {
    const [carId, setCarId] = useState('');
    const [lesseeId, setLesseeId] = useState('');
    const [durationMonths, setDurationMonths] = useState('');

    const handleLease = async () => {
        try {
            const response = await axios.post('/api/lease', {carId, lesseeId, durationMonths});
            console.log('Lease response:', response.data);
            setCarId('');
            setLesseeId('');
            setDurationMonths('');
        } catch (error) {
            console.error('Error leasing car:', error);
        }
    };

    return (
        <div>
            <Navbar/>
            <div className="container mx-auto p-5 border border-gray-300 rounded-lg">
                <h1 className="text-2xl font-bold mb-4">Lease a Car</h1>
                <div>
                    <input
                        type="text"
                        placeholder="Car ID"
                        className="w-96 py-2 px-3 mb-4 border border-gray-300 rounded-lg"
                        value={carId}
                        onChange={(e) => setCarId(e.target.value)}
                    />
                </div>
                <div>
                    <input
                        type="text"
                        placeholder="Lessee ID"
                        className="w-96 py-2 px-3 mb-4 border border-gray-300 rounded-lg"
                        value={lesseeId}
                        onChange={(e) => setLesseeId(e.target.value)}
                    />
                </div>
                <div>
                    <input
                        type="text"
                        placeholder="Duration (months)"
                        className="w-96 py-2 px-3 mb-4 border border-gray-300 rounded-lg"
                        value={durationMonths}
                        onChange={(e) => setDurationMonths(e.target.value)}
                    />
                </div>
                <div>
                    <button
                        className="bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-lg"
                        onClick={handleLease}
                    >
                        Lease Car
                    </button>
                </div>
            </div>
        </div>
    );
}
