Na Flex planu, osobe koje mogu administrirati ili moderirati vaš račun naplaćuju se kao mjesečne licence (seats). To je dio Flex računa koji najčešće iznenađuje korisnike, obično jer su licence stvorene bez da je itko primijetio.

### Što se računa kao licenca

- **Administratori**: korisnici na stranici `Users` vašeg računa s bilo kojom administrativnom dozvolom (naplata, prilagoda, analitika, moderiranje i slično). Prvi administrator, obično vlasnik računa, besplatan je; svaki dodatni administrator predstavlja licencu.
- **Moderatori**: osobe na stranici `Moderators` koje su prihvatile pozivnicu. Poslana pozivnica se ne naplaćuje. Korisnik čija je jedina dozvola moderiranje komentara naplaćuje se kao moderator, a ne kao administrator.
- **SSO administratori** i **SSO moderatori**: SSO korisnici koje vaša stranica prijavljuje s `isAccountOwner`, `isAdminAdmin` ili `isCommentModeratorAdmin` postavljenim u SSO payloadu. Naplaćuju se po istim cijenama kao obični administratori i moderatori. Pogledajte [SSO User Structure](/guide-api.html#sso-user-structure).

Licensing se deduplicira po adresi e-pošte, pa se osoba koja je i administrator i moderator, ili koja ima i SSO korisnika i prijavu na nadzornu ploču s istom adresom e-pošte, naplaćuje jednom, po višoj cijeni. Osoba s dvije različite adrese e-pošte predstavlja dvije licence.

Na računima koji se naplaćuju po mjesečno aktivnim korisnicima, SSO moderator koji se nije prijavio tijekom mjeseca ne naplaćuje se za taj mjesec.

### Licence koje niste dodali

Dvije značajke automatski stvaraju licence:

- **SSO sinkronizacija administratora.** Kada se SSO korisnik prijavi s adresom e-pošte koja odgovara jednom od korisnika vaše nadzorne ploče, dozvole administratora ili moderatora tog korisnika kopiraju se na SSO korisnika. S WordPress SSO to znači da svaki WordPress administrator može postati FastComments administrator ili moderator. Isključite to pod `Customize` opcijom `Disable Automatic Admin Migration` na pravilu prilagode.
- **Različite adrese e-pošte za istu osobu.** Ako vaša prijava na nadzornu ploču koristi jednu adresu e-pošte, a vaš SSO korisnik drugu, to su dvije licence. Korištenje iste adrese e-pošte na obje spaja ih u jednu.

### Provjera i uklanjanje licenci

Stranica [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) prikazuje brojeve vaših `Moderators`, `SSO Admins` i `SSO Moderators`, a pregled sljedećeg računa pokazuje njihov trošak. Uklonite osobe koje ne trebate sa stranica `Users` i `Moderators`, ili uklonite administratorske oznake iz svog SSO payloada. Licence se broje prilikom stvaranja računa na kraju mjeseca, pa licenca uklonjena prije toga ne naplaćuje se za taj mjesec.

Pogledajte [Adding Moderators](/guide-users.html#adding-moderators) i [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) za upravljanje obojicima.