#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Default
![Tema: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Izvorni WYSIWYG uređivač s podrškom za slike!
![Izvorni WYSIWYG uređivač s podrškom za slike](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Uređivač bogatog teksta

Ova biblioteka koristi 10tap editor za funkcionalnost uređivanja bogatog teksta, koji omogućava snažno WYSIWYG iskustvo uređivanja.

### Opcije konfiguracije

Ova biblioteka nastoji podržavati sve opcije konfiguracije definirane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), baš kao i web implementacija.

### Koncepti FastComments

Glavni koncepti koje treba razumjeti za početak su `tenantId` i `urlId`. `tenantId` je vaš FastComments.com identifikator računa. `urlId` je gdje će niti komentara
biti vezane. To može biti URL stranice, ili id proizvoda, id članka itd.

### Obavijesti za korisnike

FastComments podržava obavijesti za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obavijesti su konfigurabilne,
mogu se odjaviti globalno ili na razini obavijesti/komentara, i podržavaju pretplate na razini stranice tako da se korisnici mogu pretplatiti na niti komentara
određene stranice ili članka.

Na primjer, moguće je koristiti Secure SSO za autentifikaciju korisnika i zatim periodički ispitivati nepročitane obavijesti i slati ih korisniku.

Pogledajte [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način kako dohvatiti i prevesti nepročitane korisničke obavijesti.

### Preglednik GIF-ova

Po zadanim postavkama nije omogućeno odabiranje slika ili gifova. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podrške prijenosima slika i gifova. U ovoj biblioteci postoji Preglednik GIF-ova koji anonimizira pretraživanja i slike, jednostavno ga trebate koristiti.

### Performanse

Molimo otvorite tiket s primjerom kako reproducirati problem, uključujući uređaj koji je korišten, ako uočite bilo kakve probleme s performansama. Performanse su prioritet
u svim FastComments bibliotekama.