Pogosto je, da ima vsako testno ali razvojno okolje pri FastComments svojega podnajemnika. Vsak najemnik bi imel svojo konfiguracijo, podatke in API ključe. Konfiguracija, podatki in uporabniki se ne morejo deliti med najemniki.
Vse je izolirano. Vendar lahko super skrbniki starševskega najemnika prevzamejo identiteto uporabnikov v podrejenih najemnikih.

Obstajata dva pristopa:

- Glavni najemnik je za produkcijo, podnajemniki pa so za testna okolja.
- Glavni najemnik je preprosto za obračunavanje, vsak podnajemnik pa je za prod, test in tako naprej.

Prvi pristop je uporabnikom na splošno lažje razumljiv, vendar je to odvisno od vaše organizacije.

Najemnike lahko ustvarite [tukaj](https://eu.fastcomments.com/auth/my-account/tenants), če imate paket. Tu bi tudi super skrbniki
prevzeli identiteto uporabnikov. Najemnike je mogoče ustvariti tudi prek API-ja za bolj prilagojene/avtomatizirane nastavitve.

Ne glede na izbrani pristop boste morali v "prod" najemnik dodati moderatorje in uporabnike, ki želijo videti produkcijske podatke. Torej na primer, če želite
izbrati možnost B in imeti starševski najemnik za obračunavanje ter podnajemnika za "prod", boste želeli dodati najemnika, preklopiti na novega najemnika in dodati svoje
skrbniške in moderatorske uporabnike za podnajemnika. 

Na koncu, za pojasnilo, bo stran za moderiranje komentarjev pri možnosti B za starševski najemnik prazna.