Herbetalingen worden afgeschreven op de betaalmethode die op je account is opgeslagen. Als je nog geen kaart hebt opgeslagen, toont de Credits-pagina `Add a payment method first` met een `Go to billing`-knop. Sla een kaart op op de [Billing Info](https://fastcomments.com/auth/my-account/billing-info)-pagina en kom daarna terug.

Om tegoed toe te voegen:

1. Open [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) en klik op `Manage Credits`.
2. Kies onder `Add credit` een van de vooraf ingestelde bedragen, of kies `Other amount` en voer je eigen bedrag in.
3. Klik op de `Add`-knop. Deze toont het door jou gekozen bedrag, bijvoorbeeld `Add $25.00`.
4. Als je bank je vraagt de betaling te bevestigen, voltooi dan de controle in het venster dat opent.

Zodra de betaling is verwerkt, toont de pagina `Credit added` met je nieuwe saldo, en verschijnt de herbetaling in `Past top-ups` als `Credited`. Klik op `Add more` om opnieuw tegoed toe te voegen.

### Bedragen en limieten

De vooraf ingestelde bedragen zijn `10`, `25`, `50` en `100` in je factureringsvaluta. Een aangepast bedrag kan alles zijn van `5` tot `10.000` per herbetaling, en de exacte limieten voor jouw valuta worden onder het formulier weergegeven. Voor valuta zonder decimalen, zoals de Japanse yen, zijn de vooraf ingestelde bedragen en limieten honderd keer groter, zodat ze ongeveer hetzelfde bedrag vertegenwoordigen.

Herbetalingen zijn altijd in de factureringsvaluta van je account, dezelfde valuta als je facturen. De pagina vertelt je welke valuta wordt afgeschreven.

### Bankbevestiging (3D Secure)

Sommige banken vereisen dat je een online betaling bevestigt, meestal via je bank-app of een eenmalige code. Als dat bij jouw bank het geval is, opent er een bevestigingsvenster nadat je op `Add` hebt geklikt. Je kaart wordt pas afgeschreven, en je saldo pas bijgewerkt, nadat de bank heeft goedgekeurd.

Als je de pagina sluit voordat je de bevestiging hebt afgerond, wordt de herbetaling geregistreerd als `Pending`. De volgende keer dat je de Credits-pagina opent, controleren we de status van de betaling bij Stripe en markeren we deze als `Credited` of `Failed`. Als de bank de betaling niet goedkeurt, toont de pagina `Your bank did not approve the payment. Try again, or use a different card.` en wordt er niets afgeschreven.

### Geweigerde kaarten

Als je kaart wordt geweigerd, toont de pagina `Your card was declined. Check your card on the billing page, or try another card.` Er wordt niets afgeschreven, je saldo verandert niet, en de poging wordt vermeld in `Past top-ups` als `Failed`. Werk de kaart bij op de [Billing Info](https://fastcomments.com/auth/my-account/billing-info)-pagina en probeer het opnieuw.

Om je kaart te beschermen, kan een account een beperkt aantal herbetalingpogingen per uur doen. Als je `Too many top-up attempts. Wait a while and try again.` ziet, wacht dan en probeer het later opnieuw.

Betalingen worden verwerkt door Stripe. FastComments slaat nooit je kaartgegevens op.