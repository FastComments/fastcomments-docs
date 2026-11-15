La pagina [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), raggiungibile con il pulsante `Invoice Settings` su Billing Info, controlla cosa appare sulle tue fatture e dove vengono inviate. Solo l'`Account Owner` o un `Billing Admin` possono aprirla.

- `Company Name`: il nome del tuo account così come mostrato nella dashboard, nelle email e negli inviti al team. Non è il nome di fatturazione.
- `Billing Email`: l'indirizzo a cui vengono inviate fatture e notifiche di fatturazione. Impostalo su una casella di posta finanziaria o amministrativa se il proprietario dell'account non deve riceverle. Non modifica l'email di accesso di alcun utente.
- `Invoice Currency`: la valuta in cui vengono emesse le tue fatture. Vedi [Valute](#currencies).
- `Billing Address`: i campi `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` e `Country` stampati sulle fatture. Gli stessi campi compaiono nel modulo Billing Info quando salvi una carta; entrambi scrivono nello stesso posto.
- `VAT / Tax ID`: facoltativo. Stampato sulle tue fatture.

Fai clic su `Save Changes`. La pagina conferma con `Invoice settings updated successfully.` e le modifiche si applicano alla prossima fattura. Le fatture esistenti non vengono modificate.

### Partite IVA e ID fiscali

Seleziona il tuo `Country` di fatturazione prima di inserire l'ID, perché il paese determina il tipo di ID: un numero IVA UE per qualsiasi stato membro dell'UE, un numero IVA del Regno Unito per il Regno Unito, un ABN per l'Australia, un numero GST/HST per il Canada, un EIN per gli Stati Uniti, e così via per gli altri paesi supportati da Stripe.

L'ID viene validato quando salvi. Se non è valido per il paese selezionato, la pagina te lo indica e l'ID non viene salvato, mentre le altre impostazioni sì. Se il tuo paese non è uno di quelli per cui possiamo registrare un ID fiscale, la pagina te lo comunica.

Se hai un indirizzo di fatturazione nell'UE, le fatture Flex vengono emesse con IVA allo 0% come operazione a reverse charge, e il tuo numero IVA compare su di esse una volta salvato, così il tuo commercialista può gestire l'IVA dal tuo lato.