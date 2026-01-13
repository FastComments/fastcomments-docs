#### Tema: Erebus
![Tema: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Tema: Default
![Tema: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Nativni WYSIWYG editor sa podrškom za slike!
![Nativni WYSIWYG editor sa podrškom za slike](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Editor za formatirani tekst

Ova biblioteka koristi 10tap editor za funkcionalnost uređivanja formatiranog teksta, koji pruža snažno WYSIWYG iskustvo uređivanja.

### Opcije konfiguracije

Ova biblioteka teži da podrži sve opcije konfiguracije definisane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), isto kao i web implementacija.

### Koncepti FastComments-a

Glavni koncepti koje treba znati da biste počeli su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` je mesto na koje će biti vezane niti komentara. To može biti URL stranice, ili id proizvoda, id članka, itd.

### Obaveštenja korisnika

FastComments podržava obaveštenja za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obaveštenja su konfigurisana, moguće je odjaviti se globalno ili na nivou obaveštenja/komentara, i podržavaju pretplate na nivou stranice tako da se korisnici mogu pretplatiti na niti određenih stranica ili članaka.

Na primer, moguće je koristiti Secure SSO za autentifikaciju korisnika, a zatim periodično proveravati nepročitana obaveštenja i slati ih korisniku.

Pogledajte [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za primer kako dobiti i prevesti nepročitana korisnička obaveštenja.

### Pregledač GIF-ova

Po defaultu, izbor slika ili gifova nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podrške za otpremanje slika i gifova. U ovoj biblioteci postoji Pregledač GIF-ova koji anonimizuje pretrage i slike, potrebno je samo da ga koristite.

### Performanse

Molimo otvorite tiket sa primerom kako reprodukovati problem, uključujući uređaj koji ste koristili, ako uočite bilo kakve probleme sa performansama. Performanse su prioritet u svim FastComments bibliotekama.