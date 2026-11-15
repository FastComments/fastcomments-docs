Kredit je novac koji se nalazi na vašem FastComments nalogu, u valuti vaše naplate. Automatski se primenjuje na vaše buduće fakture, uključujući obnove pretplate i Flex korišćenje, pre nego što se naplati vaša kartica.

Kredit je preplata, a ne popust ili sistem poena. Dodavanje kredita od `$50` znači da će vaše naredne fakture od `$50` biti plaćene iz tog kredita umesto sa vaše kartice. Stanje se čuva kao kredit na vašem nalogu u Stripe-u, našem procesoru plaćanja, i ostaje na vašem nalogu dok ga neka faktura ne iskoristi.

Neki od razloga za održavanje kreditnog stanja:

- Preplata perioda unapred, na primer kada finansijski sektor preferira jednu veću uplatu umesto mesečnog zaduživanja kartice.
- Održavanje rezerve na Flex planu tako da je zauzet mesec pokriven pre nego što faktura stigne.
- Plaćanje karticom jednom, a zatim obnove crpe sredstva iz stanja.

### Gde da ga pronađete

Otvorite [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) i kliknite na `Manage Credits`, pored `View Invoices` i `Invoice Settings`. Ovo otvara stranicu `Credits`, koja prikazuje vaše trenutno `Credit balance`, formu `Add credit` i vaše `Past top-ups`.

Kada imate stanje, stranica `Billing Info` takođe prikazuje liniju `Credit balance` kako biste ga videli bez otvaranja stranice Credits.

### Ko može da doda kredit

Samo `Account Owner` ili korisnik sa dozvolom `Billing Admin` može da doda kredit, isti korisnici koji mogu da menjaju plan ili način plaćanja. Ostali administratori ne vide dugme `Manage Credits`. Pogledajte [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) za to kako da dodelite dozvolu.