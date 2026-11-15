Na Flex planu, osobe koje mogu da administriraju ili moderiraju vaš nalog se naplaćuju kao mesečne licence (seats). Ovo je deo Flex računa koji najčešće iznenađuje korisnike, obično zato što su licence napravljene a da to niko nije primetio.

### Šta Se Računa Kao Licenca

- **Administratori**: korisnici na stranici `Users` vašeg naloga sa bilo kojom administrativnom dozvolom (naplata, prilagođavanje, analitika, moderacija i tako dalje). Prvi administrator, obično vlasnik naloga, je besplatan; svaki dodatni administrator je jedna licenca.
- **Moderatori**: osobe na stranici `Moderators` koje su prihvatile pozivnicu. Neprihvaćena pozivnica se ne naplaćuje. Korisnik čija je jedina dozvola moderacija komentara naplaćuje se kao moderator, a ne kao administrator.
- **SSO Administratori** i **SSO Moderatori**: SSO korisnici koje vaš sajt prijavljuje sa `isAccountOwner`, `isAdminAdmin` ili `isCommentModeratorAdmin` postavljenim u SSO payload-u. Naplaćuju se po istim cenama kao obični administratori i moderatori. Pogledajte [SSO User Structure](/guide-api.html#sso-user-structure).

Licence se deduplikuju po email adresi, tako da jedna osoba koja je i administrator i moderator, ili koja ima i SSO korisnika i prijavu na kontrolnu tablu sa istim emailom, naplaćuje se jednom, po višoj ceni. Osoba sa dve različite email adrese su dve licence.

Na nalozima koji se naplaćuju po mesečno aktivnim korisnicima, SSO moderator koji se nije prijavio tokom meseca se ne naplaćuje za taj mesec.

### Licence Koje Niste Dodali

Dve funkcionalnosti automatski prave licence:

- **SSO sinhronizacija administratora.** Kada se SSO korisnik prijavi sa emailom koji se poklapa sa jednim od korisnika vaše kontrolne table, administratorske ili moderatorske dozvole tog korisnika se kopiraju na SSO korisnika. Sa WordPress SSO ovim znači da svaki WordPress administrator može postati FastComments administrator ili moderator. Isključite ovo pod `Customize` opcijom `Disable Automatic Admin Migration` na vašem pravilu prilagođavanja.
- **Različiti emailovi za istu osobu.** Ako vaša prijava na kontrolnu tablu koristi jedan email, a vaš SSO korisnik drugi, to su dve licence. Korišćenje istog emaila na oba ih spaja.

### Provera I Uklanjanje Licenci

Stranica [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) prikazuje broj vaših `Moderators`, `SSO Admins` i `SSO Moderators`, a pregled sledeće fakture pokazuje koliko koštaju. Uklonite osobe koje ne trebate sa stranica `Users` i `Moderators`, ili uklonite administratorske oznake iz vašeg SSO payload-a. Licence se broje prilikom kreiranja fakture na kraju meseca, tako da licenca uklonjena pre toga se ne naplaćuje za taj mesec.

Pogledajte [Adding Moderators](/guide-users.html#adding-moderators) i [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) za upravljanje oboma.