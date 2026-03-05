Kada FastComments pošalje e-poruku koja se odbije (bounce) ili je primalac označi kao neželjenu poštu, provajder e-pošte dodaje tu adresu na listu potiskivanja (suppression list). FastComments sinhronizuje ove liste potiskivanja svakog dana kako se dodatne e-poruke ne bi slale na adrese koje ih ne mogu primiti.

Korisnici i moderatori čije su adrese e-pošte potisnute neće primati nikakva obaveštenja putem e-pošte, uključujući obaveštenja o odgovorima, obaveštenja o pomenima, administratorske upozorenja i sažetke (digest) putem e-pošte. Pored pogođenih korisnika i moderatora u administratorskom UI pojaviće se crveni bedž "E-mail potisnut".

#### Pregled potisnutih e-pošta

Administrator zakupca sa dozvolom Upravljanje podacima može da pregleda potisnute e-pošte na stranici
[Potisnute e-pošte](https://fastcomments.com/auth/my-account/suppressed-emails), u okviru Upravljanja podacima.

Stranica prikazuje tabelu svih potisnutih adresa e-pošte povezanih sa korisnicima, moderatorima i komentatorima vašeg zakupca. Možete filtrirati po adresi e-pošte koristeći polje za pretragu.

#### Uklanjanje potiskivanja

Da biste uklonili potiskivanje, kliknite na **Ukloni** dugme pored unosa u tabeli. Bićete preusmereni na stranicu za potvrdu koja prikazuje detalje potiskivanja. Kliknite **Potvrdi uklanjanje** da nastavite.

Kada se potiskivanje ukloni, FastComments kontaktira provajdera e-pošte kako bi otključao adresu i uklanja oznaku potiskivanja sa svih povezanih zapisa korisnika i moderatora.

#### Ograničenja stope (Rate Limits)

Kako bi se sprečila zloupotreba, uklanjanja su ograničena:

- Svaka adresa e-pošte može biti uklonjena iz liste potisnutih samo jednom u roku od 30 dana.
- Svaki zakupac može izvršiti do 5 uklanjanja po kalendarskom mesecu.

Ako se potiskivanje ponovo pojavi nakon uklanjanja, to znači da je adresa e-pošte ponovo odbijena ili prijavljena kao neželjena pošta. U tom slučaju, osnovni problem isporučivosti treba rešiti pre pokušaja novog uklanjanja.