Ogni addebito, che si tratti di un rinnovo del piano, di un mese Flex o di un cambio di piano, genera una fattura. Le fatture fungono anche da ricevute una volta pagate.

### Come trovare le fatture

Apri [Billing History](https://fastcomments.com/auth/my-account/billing-history), oppure fai clic su `View Invoices` nella pagina Billing Info. La tabella elenca ogni fattura con la sua data, il periodo di fatturazione, il numero di fattura, la descrizione, il totale, lo stato (`Paid`, `Unpaid`, `Draft` o `Void`) e la data di scadenza, dieci per pagina, dalla più recente.

Per ogni fattura:

- `View` apre la fattura nella pagina ospitata di Stripe, dove puoi anche scaricarla.
- `PDF` scarica direttamente il PDF.
- `Pay Now` appare al posto di `View` su una fattura non pagata e ti porta alla pagina di pagamento di Stripe per quella fattura, dove puoi pagarla direttamente.

Le fatture vengono inoltre inviate via email al tuo indirizzo di fatturazione al momento della loro creazione. Per far sì che vengano inviate a una casella di posta finanziaria invece che al proprietario dell'account, imposta il `Billing Email` in [Invoice Settings](#invoice-settings).

### Cosa contiene la fattura

La fattura mostra il nome e l'indirizzo di fatturazione e la partita IVA / Tax ID di [Invoice Settings](#invoice-settings), le voci di dettaglio, l'eventuale [credito](#credits) prepagato applicato e l'importo addebitato sulla tua carta. Le fatture Flex elencano una voce per ogni tipo di utilizzo, vedi [How Flex Usage Is Measured](#how-flex-usage-is-measured). Per gli indirizzi di fatturazione nell'Unione Europea, le fatture Flex vengono emesse con IVA allo 0% e con la nota in fondo `Reverse Charge Transaction: Customer to pay VAT`.

Le modifiche alle impostazioni delle fatture si applicano alle fatture create dopo il salvataggio. Una fattura già emessa non può essere modificata, quindi imposta i dati della tua azienda e la partita IVA / Tax ID prima della prossima data di fatturazione.

### Account fatturati altrove

Se la tua fatturazione è gestita esternamente, ad esempio tramite fatturazione concordata con l'assistenza, le fatture ti vengono inviate per il pagamento invece di essere addebitate su una carta e potrebbero non apparire in Billing History. Gli account installati tramite Shopify o BigCommerce sono fatturati da quella piattaforma, e la pagina Billing Info contiene un link alla pagina del piano corrispondente. Gli sub-account fatturati tramite un account principale non hanno fatture proprie; vedi [When You Are Charged](#when-you-are-charged).