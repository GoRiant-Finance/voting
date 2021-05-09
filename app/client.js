const anchor = require('@project-serum/anchor');

anchor.setProvider(anchor.Provider.local("https://devnet.solana.com"));

async function main() {

} 

console.log('Running client.');
main().then(() => console.log('Success'));