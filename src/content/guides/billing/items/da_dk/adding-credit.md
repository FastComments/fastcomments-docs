Indbetalinger trækkes fra den betalingsmetode, der er gemt på din konto. Hvis du endnu ikke har gemt et kort, viser Credits-siden `Add a payment method first` med en `Go to billing`-knap. Gem et kort på [Billing Info](https://fastcomments.com/auth/my-account/billing-info)-siden, og vend derefter tilbage.

Sådan tilføjer du kredit:

1. Åbn [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info), og klik på `Manage Credits`.
2. Under `Add credit` vælger du et af de forudindstillede beløb, eller vælger `Other amount` og indtaster dit eget.
3. Klik på `Add`-knappen. Den viser det beløb, du valgte, f.eks. `Add $25.00`.
4. Hvis din bank beder dig bekræfte betalingen, skal du gennemføre kontrollen i det vindue, der åbnes.

Når betalingen går igennem, viser siden `Credit added` med din nye saldo, og indbetalingen vises i `Past top-ups` som `Credited`. Klik på `Add more` for at indbetale igen.

### Beløb og grænser

Forudindstillingerne er `10`, `25`, `50` og `100` i din faktureringsvaluta. Et brugerdefineret beløb kan være alt fra `5` til `10,000` pr. indbetaling, og de præcise grænser for din valuta vises under formularen. For valutaer uden decimaler, f.eks. japanske yen, er forudindstillingerne og grænserne hundrede gange større, så de repræsenterer omtrent det samme beløb.

Indbetalinger sker altid i din kontos faktureringsvaluta, samme valuta som dine fakturaer. Siden fortæller dig, hvilken valuta der trækkes fra.

### Bankbekræftelse (3D Secure)

Nogle banker kræver, at du bekræfter en onlinebetaling, normalt via din bankapp eller en engangskode. Hvis din bank gør det, åbnes et bekræftelsesvindue, efter du har klikket på `Add`. Dit kort opkræves først, og din saldo opdateres først, efter at banken har godkendt.

Hvis du lukker siden, inden du har afsluttet bekræftelsen, registreres indbetalingen som `Pending`. Næste gang du åbner Credits-siden, tjekker vi betalingens status hos Stripe og markerer den som `Credited` eller `Failed` i overensstemmelse hermed. Hvis banken ikke godkender betalingen, viser siden `Your bank did not approve the payment. Try again, or use a different card.`, og der opkræves intet.

### Afviste kort

Hvis dit kort afvises, viser siden `Your card was declined. Check your card on the billing page, or try another card.` Der opkræves intet, din saldo ændres ikke, og forsøget vises i `Past top-ups` som `Failed`. Opdater kortet på [Billing Info](https://fastcomments.com/auth/my-account/billing-info)-siden, og prøv igen.

For at beskytte dit kort kan en konto kun foretage et begrænset antal indbetalingsforsøg pr. time. Hvis du ser `Too many top-up attempts. Wait a while and try again.`, så vent og prøv igen senere.

Betalinger behandles af Stripe. FastComments gemmer aldrig dine kortoplysninger.