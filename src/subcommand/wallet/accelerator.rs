use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Accelerator {
  address: Address<NetworkUnchecked>,
  tx: Txid,
  #[arg(long, help = "Don't sign or broadcast transaction")]
  pub(crate) dry_run: bool,
  #[arg(long, help = "Use fee rate of <FEE_RATE> sats/vB")]
  fee_rate: FeeRate,
}

impl Accelerator {
  pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
    let client = wallet.bitcoin_client();

    let address = self
      .address
      .clone()
      .require_network(wallet.chain().network())?;

    ensure!(
      wallet.has_address(&address)?,
      "The `{address}` address does not belong to your wallet address"
    );

    let txr = client.get_transaction(&self.tx, None)?;

    ensure!(
      txr.info.confirmations == 0,
      "The transaction has already been confirmed"
    );

    let tx = client.get_raw_transaction(&self.tx, None)?;

    let fee_rate = match txr.fee {
      Some(fee) => fee.to_sat() as f64 / tx.size() as f64,
      None => 0.0,
    };

    ensure!(
      fee_rate < self.fee_rate.n(),
      "The transaction fee rate is already higher than the specified fee rate"
    );

    Ok(Some(Box::new(())))
  }
}
