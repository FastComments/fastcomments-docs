#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Default
![Tema: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Nativni WYSIWYG uređivač sa podrškom za slike!
![Nativni WYSIWYG uređivač sa podrškom za slike](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Uređivač bogatog teksta

Ova biblioteka koristi 10tap editor za funkcionalnost uređivanja bogatog teksta, što pruža moćno WYSIWYG uređivačko iskustvo.

### Opcije konfiguracije

Ova biblioteka ima za cilj da podrži sve opcije konfiguracije definisane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), isto kao i web implementacija.

### Koncepti FastComments

Glavni koncepti koje treba razumjeti da biste počeli su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` je mjesto na koje će biti vezane niti komentara. To može biti URL stranice, ili id proizvoda, id članka, itd.

### Korisnička obaveštenja

FastComments podržava obaveštenja za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obaveštenja su konfigurabilna, moguće je odustati od njih globalno ili na nivou obaveštenja/komentara, i podržavaju pretplate na nivou stranice tako da se korisnici mogu pretplatiti na niti određenih stranica ili članaka.

Na primer, moguće je koristiti Secure SSO za autentifikaciju korisnika, a zatim periodično proveravati nepročitana obaveštenja i slati ih korisniku.

Pogledajte [primer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za to kako dobiti i prevesti nepročitana korisnička obaveštenja.

### GIF pregledač

Po defaultu, nijedna selekcija slike ili gif-a nije omogućena. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za to kako podržati otpremanje slika i gif-ova. U ovoj biblioteci postoji GIF pregledač koji anonimizuje pretrage i slike koje pruža, dovoljno je samo da ga koristite.

### Performanse

Molimo otvorite tiket sa primjerom za reprodukciju, uključujući uređaj koji koristite, ako primijetite bilo kakve probleme sa performansama. Performanse su primarni prioritet u svim FastComments bibliotekama.