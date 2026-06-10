#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Default
![Tema: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Nativni WYSIWYG uređivač s podrškom za slike!
![Nativni WYSIWYG uređivač s podrškom za slike](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Uređivač bogatog teksta

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za uređivanje bogatog teksta, što pruža snažno WYSIWYG iskustvo uređivanja. Isti uređivač pokreće iOS, Android i web (putem `react-native-web`), tako da se composer ponaša dosljedno na svakoj platformi s jednom implementacijom.

`react-native-enriched` zahtijeva React Native New Architecture (Fabric) na nativnim platformama, i bundler koji rješava uvjete `exports` paketa (Metro s package exports / RN 0.72+). Podrška za web je trenutno eksperimentalna.

### Opcije konfiguracije

Ova biblioteka nastoji podržati sve opcije konfiguracije definirane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), baš kao i web implementacija.

### Koncepti FastComments

Glavni koncepti koje trebate znati za početak su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com računa. `urlId` određuje čemu će biti vezane niti komentara. To može biti URL stranice, id proizvoda, id članka, itd.

### Obavijesti korisnika

FastComments podržava obavijesti za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obavijesti su konfigurabilne, mogu se isključiti globalno ili na razini pojedinačne obavijesti/komentara, i podržavaju pretplate na razini stranice tako da se korisnici mogu pretplatiti na niti određene stranice ili članka.

Na primjer, moguće je koristiti Secure SSO za autentikaciju korisnika, a zatim periodički provjeravati nepročitane obavijesti i slati ih korisniku.

Pogledajte [primjer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način kako dohvatiti i prevesti nepročitane korisničke obavijesti.

### Preglednik GIF-ova

Prema zadanim postavkama, odabir slika ili GIF-ova nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podrške za prijenose slika i GIF-ova. U ovoj biblioteci postoji Gif Browser koji anonimizira pretraživanja i slike, samo ga trebate koristiti.

### Performanse

Molimo otvorite ticket s primjerom reprodukcije, uključujući uređaj koji ste koristili, ako uočite bilo kakve probleme s performansama. Performanse su prioritet u svim FastComments bibliotekama.