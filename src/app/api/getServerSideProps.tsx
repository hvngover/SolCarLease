import axios from "axios";

export async function getServerSideProps() {
    let balance = null;
    try {
        const response = await axios.get('/api/get-account-balance?pubkey=9hgKD435MdcWndPwN8hDrhqoSveToJbzcgKbJkDU4jD6');
        const data = response.data;
        balance = data.balance;
    } catch (error) {
        console.error('Error fetching balance:', error);
    }

    return {
        props: {
            balance
        }
    };
}
