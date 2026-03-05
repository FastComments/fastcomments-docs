Kada emailovi koje šalje FastComments odbiju ili označe primatelji kao spam, provajder emaila dodaje tu adresu na listu suzbijanja. FastComments sinhronizuje ove liste suzbijanja svakog dana kako bi se spriječilo slanje daljih emailova na adrese koje ih ne mogu primiti.

Korisnici i moderatori čije su email adrese suzbijene neće primati nikakve email notifikacije, uključujući notifikacije o odgovorima, notifikacije o pomenima, administrativna upozorenja i digest emailove. Pored pogođenih korisnika i moderatora u administratorskom sučelju pojaviće se crveni bedž "Email blokiran".

#### Pregled blokiranih emailova

Administratori tenant-a sa dozvolom "Upravljanje podacima" mogu pregledati blokirane emailove na stranici [Blokirani emailovi](https://fastcomments.com/auth/my-account/suppressed-emails), u okviru "Upravljanje podacima".

Stranica prikazuje tabelu svih blokiranih email adresa povezanih sa korisnicima, moderatorima i komentatorima vašeg tenanta. Možete filtrirati po email adresi koristeći polje za pretragu.

#### Uklanjanje blokade

Da biste uklonili blokadu, kliknite **Ukloni** dugme pored unosa u tabeli. Bićete preusmjereni na stranicu za potvrdu koja prikazuje detalje blokade. Kliknite **Potvrdi uklanjanje** da biste nastavili.

Kada se blokada ukloni, FastComments kontaktira provajdera emaila da deblokira adresu i uklanja flag suzbijanja sa svih povezanih zapisa korisnika i moderatora.

#### Ograničenja

Da bi se spriječila zloupotreba, uklanjanja su ograničena:

- Svaka email adresa može biti deblokirana samo jednom u 30 dana.
- Svaki tenant može izvršiti do 5 uklanjanja po kalendarskom mjesecu.

Ako se blokada ponovo pojavi nakon uklanjanja, to znači da je email adresa ponovo odbijena ili prijavljena kao spam. U tom slučaju, osnovni problem dostavljivosti treba riješiti prije ponovnog pokušaja uklanjanja.