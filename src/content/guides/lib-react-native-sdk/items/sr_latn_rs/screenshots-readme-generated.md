---
#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Default
![Tema: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Nativni WYSIWYG uređivač sa podrškom za slike!
![Nativni WYSIWYG uređivač sa podrškom za slike](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Uređivač bogatog teksta

Ova biblioteka koristi 10tap editor za funkcionalnost uređivanja bogatog teksta, koji pruža moćno WYSIWYG iskustvo uređivanja.

### Opcije konfiguracije

Ova biblioteka teži da podrži sve opcije konfiguracije definisane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), isto kao i web implementacija.

### Koncepti FastComments

Glavni koncepti koje treba razumeti da biste počeli su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` je za šta će biti vezane niti komentara. To može biti URL stranice, ili ID proizvoda, ID članka, itd.

### Obaveštenja korisnika

FastComments podržava obaveštenja za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obaveštenja su konfigurisanja,
mogu se isključiti globalno ili na nivou obaveštenja/komentara, i podržavaju pretplate na nivou stranice tako da se korisnici mogu pretplatiti na niti
konkretne stranice ili članka.

Na primer, moguće je koristiti Secure SSO za autentifikaciju korisnika i zatim periodično proveravati nepročitana obaveštenja i slati ih korisniku.

Pogledajte [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za to kako dobiti i prevesti nepročitana korisnička obaveštenja.

### Gif pregledač

Podrazumevano, izbor slika ili gifova nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za to kako
podržati otpremanje slika i gifova. U ovoj biblioteci postoji Gif pregledač koji anonimizuje pretrage i slike koje pruža, dovoljno je samo da ga koristite.

### Performanse

Molimo otvorite tiket sa primerom za reprodukciju, uključujući korišćeni uređaj, ako uočite bilo kakve probleme sa performansama. Performanse su prioritet
u svim FastComments bibliotekama.
---