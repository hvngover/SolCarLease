import React from 'react';

interface CarProps {
    id: number;
    price: number;
    owner: string;
}

const CarComponent: React.FC<CarProps> = ({ id, price, owner }) => {
    return (
        <div className="border border-gray-300 p-4 rounded-md shadow-md mb-4">
            <h2 className="text-xl font-bold mb-2">Car {id}</h2>
            <p><span className="font-bold">ID:</span> {id}</p>
            <p><span className="font-bold">Price:</span> {price}</p>
            <p><span className="font-bold">Owner:</span> {owner}</p>
        </div>
    );
};

export default CarComponent;
