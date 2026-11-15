Kredit je novac koji se nalazi na vašem FastComments nalogu, u valuti naplate. Automatski se primenjuje na vaše buduće fakture, uključujući obnove pretplate i Flex korišćenje, pre nego što se naplati vaša kartica.

Kredit je pretplata unapred, a ne popust ili sistem poena. Dodavanje `$50` kredita znači da će se vaših narednih `$50` faktura platiti iz tog kredita umesto sa kartice. Stanje se čuva kao kredit na vašem nalogu u Stripe-u, našem procesoru plaćanja, i ostaje na vašem nalogu dok ga faktura ne iskoristi.

Neki od razloga za održavanje kreditnog stanja:

- Plaćanje unapred za period, na primer kada finansijski odeljek preferira jednu veću uplatu umesto mesečnog naplaćivanja sa kartice.
- Održavanje rezerve na Flex planu tako da je zauzet mesec pokriven pre nego što faktura stigne.
- Plaćanje karticom jednom, a zatim obnove se naplaćuju sa stanja.

### Gde da ga pronađete

Otvorite [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) i kliknite `Manage Credits`, pored `View Invoices` i `Invoice Settings`. Ovo otvara stranicu `Credits`, koja prikazuje vaše trenutno `Credit balance`, formu `Add credit` i vaše `Past top-ups`.

Kada imate stanje, stranica `Billing Info` takođe prikazuje liniju `Credit balance` kako biste ga videli bez otvaranja stranice Credits.

### Ko može da doda kredit

Samo `Account Owner` ili korisnik sa dozvolom `Billing Admin` može da doda kredit, isti korisnici koji mogu da menjaju plan ili način plaćanja. Ostali administratori ne vide dugme `Manage Credits`. Pogledajte [Ko može da upravlja naplatom](#who-can-manage-billing).