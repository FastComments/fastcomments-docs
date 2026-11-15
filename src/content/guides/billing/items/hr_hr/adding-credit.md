Nadoplate se naplaćuju s načina plaćanja spremljenog na vašem računu. Ako još niste spremili karticu, stranica Credits prikazuje `Add a payment method first` s gumbom `Go to billing`. Spremite karticu na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info), a zatim se vratite.

Za dodavanje kredita:

1. Otvorite [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) i kliknite `Manage Credits`.
2. Pod `Add credit` odaberite jedan od unaprijed definiranih iznosa ili odaberite `Other amount` i unesite vlastiti.
3. Kliknite gumb `Add`. Prikazuje iznos koji ste odabrali, na primjer `Add $25.00`.
4. Ako vaša banka traži potvrdu plaćanja, dovršite provjeru u prozoru koji se otvori.

Kada plaćanje prođe, stranica prikazuje `Credit added` s vašim novim stanjem, a nadoplata se pojavljuje u `Past top-ups` kao `Credited`. Kliknite `Add more` za ponovnu nadoplatu.

### Iznosi i ograničenja

Unaprijed definirani iznosi su `10`, `25`, `50` i `100` u valuti vaše naplate. Prilagođeni iznos može biti bilo što od `5` do `10,000` po nadoplati, a točna ograničenja za vašu valutu prikazana su ispod obrasca. Za valute bez decimalnih mjesta, poput japanskog jena, unaprijed definirani iznosi i ograničenja su sto puta veći kako bi predstavljali otprilike isti novčani iznos.

Nadoplate su uvijek u valuti naplate vašeg računa, istoj valuti kao i vaši računi. Stranica vam govori koja će valuta biti naplaćena.

### Potvrda banke (3D Secure)

Neke banke zahtijevaju da potvrdite internetsko plaćanje, obično putem svoje bankovne aplikacije ili jednokratnog koda. Ako vaša banka to zahtijeva, nakon klika na `Add` otvara se prozor za potvrdu. Vaša kartica se naplaćuje, a vaše stanje ažurira, tek nakon što banka odobri.

Ako zatvorite stranicu prije završetka potvrde, nadoplata se bilježi kao `Pending`. Sljedeći put kada otvorite stranicu Credits provjeravamo status plaćanja sa Stripeom i označavamo ga kao `Credited` ili `Failed`. Ako banka ne odobri plaćanje, stranica prikazuje `Your bank did not approve the payment. Try again, or use a different card.` i ništa se ne naplaćuje.

### Odbijene kartice

Ako je vaša kartica odbijena, stranica prikazuje `Your card was declined. Check your card on the billing page, or try another card.` Ništa se ne naplaćuje, vaše se stanje ne mijenja, a pokušaj je naveden u `Past top-ups` kao `Failed`. Ažurirajte karticu na stranici [Billing Info](https://fastcomments.com/auth/my-account/billing-info) i pokušajte ponovno.

Radi zaštite vaše kartice, račun može napraviti ograničen broj pokušaja nadoplate po satu. Ako vidite `Too many top-up attempts. Wait a while and try again.`, pričekajte i pokušajte kasnije.

Plaćanja obrađuje Stripe. FastComments nikada ne pohranjuje podatke vaše kartice.