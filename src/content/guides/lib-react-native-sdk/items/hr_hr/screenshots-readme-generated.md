#### Tema: Erebus
![Tema: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Tema: Default
![Tema: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Izvorni WYSIWYG uređivač s podrškom za slike!
![Izvorni WYSIWYG uređivač s podrškom za slike](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Uređivač bogatog teksta

Ova biblioteka koristi 10tap uređivač za funkcionalnost uređivanja bogatog teksta, koji pruža snažno WYSIWYG iskustvo uređivanja.

### Opcije konfiguracije

Ova biblioteka nastoji podržati sve opcije konfiguracije definirane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), baš kao i web implementacija.

### Koncepti FastComments

Glavni pojmovi koje trebate znati za početak su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com računa. `urlId` je mjesto na koje će biti vezane niti komentara. To može biti URL stranice, ID proizvoda, ID članka itd.

### Obavijesti korisnika

FastComments podržava obavijesti za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obavijesti su konfigurabilne, korisnik se može odjaviti globalno ili na razini pojedine obavijesti/komentara, i podržavaju pretplate na razini stranice tako da se korisnici mogu pretplatiti na niti određene stranice ili članka.

Na primjer, moguće je koristiti Secure SSO za autentikaciju korisnika, a zatim periodički provjeravati nepročitane obavijesti i slati ih korisniku.

Pogledajte [primjer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način dohvaćanja i prevođenja nepročitanih obavijesti korisnika.

### Preglednik GIF-ova

Po zadanim postavkama nije omogućeno odabiranje slika ili gifova. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podrške za prijenose slika i gifova. U ovoj biblioteci postoji Gif preglednik koji anonimizira pretraživanja i slike koje se dostave — samo ga trebate koristiti.

### Performanse

Ako primijetite probleme s izvedbom, otvorite ticket s primjerom za reproduciranje, uključujući korišteni uređaj. Performanse su prioritet u svim FastComments bibliotekama.