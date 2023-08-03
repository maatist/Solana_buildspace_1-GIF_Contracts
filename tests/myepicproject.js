const anchor = require('@project-serum/anchor');

// Esto lo usamos mas tarde
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  // Se crea y setea el provider. Lo hicimos antes pero hay que updatearlo para poder comunicarse con el Front
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;
	
  // Se crea el keypair para que el programa lo use
  const baseAccount = anchor.web3.Keypair.generate();

  // Llamamos start_stuff_off, pass it the params it needs!
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Traemos los datos desde la cuenta
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // Ahora le pasamos un link de GIF
  await program.rpc.addGif("https://media2.giphy.com/media/v1.Y2lkPTc5MGI3NjExeW9wbHZwYjM4cjV5cHZyaHNkYW40bWRrZTkybXF3eG94czdkczAzbiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/gk3R16JhLP8RUka2nD/giphy.gif", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // llamamos a la cuenta
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // Entramos a la lista de GIFs de la cuenta
  console.log('👀 GIF List', account.gifList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();