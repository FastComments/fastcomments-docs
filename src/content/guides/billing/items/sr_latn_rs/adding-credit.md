Nadoplate se naplaćuju preko plaćanja sačuvanog na vašem nalogu. Ako još niste sačuvali karticu, stranica Credits prikazuje `Add a payment method first` sa dugmetom `Go to billing`. Sačuvajte karticu na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info), a zatim se vratite.

Da dodate kredit:

1. Otvorite [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) i kliknite `Manage Credits`.
2. Pod `Add credit`, izaberite jedan od ponuđenih iznosa ili odaberite `Other amount` i unesite svoj.
3. Kliknite na dugme `Add`. Ono prikazuje iznos koji ste odabrali, na primer `Add $25.00`.
4. Ako vaša banka traži potvrdu plaćanja, završite proveru u prozoru koji se otvori.

Kada plaćanje prođe, stranica prikazuje `Credit added` sa vašim novim stanjem, a nadoplata se pojavljuje u `Past top-ups` kao `Credited`. Kliknite `Add more` da ponovo dopunite.

### Iznosi i ograničenja

Ponuđeni iznosi su `10`, `25`, `50` i `100` u valuti vašeg naplatnog naloga. Prilagođeni iznos može biti bilo šta od `5` do `10,000` po nadoplati, a tačna ograničenja za vašu valutu prikazana su ispod obrasca. Za valute bez decimalnih mesta, kao što je japanski jen, ponuđeni iznosi i ograničenja su sto puta veća kako bi predstavljala otprilike istu vrednost.

Nadoplate su uvek u naplatnoj valuti vašeg naloga, istoj valuti kao i vaši računi. Stranica vam govori koja će valuta biti naplaćena.

### Potvrda banke (3D Secure)

Neke banke zahtevaju da potvrdite online plaćanje, obično putem vaše bankarske aplikacije ili jednokratnog koda. Ako vaša banka to zahteva, prozor za potvrdu se otvara nakon što kliknete `Add`. Vaša kartica se naplaćuje, a stanje ažurira, tek nakon što banka odobri.

Ako zatvorite stranicu pre završetka potvrde, nadoplata se beleži kao `Pending`. Sledeći put kada otvorite stranicu Credits proveravamo status plaćanja sa Stripe-om i označavamo ga kao `Credited` ili `Failed` u skladu s tim. Ako banka ne odobri plaćanje, stranica prikazuje `Your bank did not approve the payment. Try again, or use a different card.` i ništa se ne naplaćuje.

### Odbijene kartice

Ako je vaša kartica odbijena, stranica prikazuje `Your card was declined. Check your card on the billing page, or try another card.` Ništa se ne naplaćuje, vaše se stanje ne menja, a pokušaj je naveden u `Past top-ups` kao `Failed`. Ažurirajte karticu na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info) i pokušajte ponovo.

Da zaštitimo vašu karticu, nalog može napraviti ograničen broj pokušaja nadoplate po satu. Ako vidite `Too many top-up attempts. Wait a while and try again.`, sačekajte i pokušajte kasnije.

Plaćanja obrađuje Stripe. FastComments nikada ne čuva podatke vaše kartice.