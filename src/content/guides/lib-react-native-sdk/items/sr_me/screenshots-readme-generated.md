#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Podrazumevano
![Tema: Podrazumevano](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Nativni WYSIWYG uređivač sa podrškom za slike!
![Nativni WYSIWYG uređivač sa podrškom za slike](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Uređivač bogatog teksta

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za uređivanje bogatog teksta, što obezbjeđuje moćno WYSIWYG iskustvo uređivanja. Isti uređivač pokreće iOS, Android i web (putem `react-native-web`), tako da se kompozitor ponaša dosljedno na svim platformama uz jednu implementaciju.

`react-native-enriched` zahtijeva React Native New Architecture (Fabric) na nativnim platformama, i bundler koji rješava uslove `exports` paketa (Metro sa package exports / RN 0.72+). Podrška za web je trenutno eksperimentalna.

### Opcije konfiguracije

Ova biblioteka ima za cilj da podrži sve opcije konfiguracije definisane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), isto kao i web implementacija.

### Koncepti FastComments-a

Glavni koncepti koje treba razumjeti da biste počeli su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` je za šta će biti vezane niti komentara. To može biti URL stranice, ili id proizvoda, id članka, itd.

### Obaveštenja korisnika

FastComments podržava obaveštenja za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obaveštenja su konfigurabilna,
mogu se isključiti globalno ili na nivou pojedinačnog obaveštenja/komentara, i podržavaju pretplate na nivou stranice tako da se korisnici mogu pretplatiti na niti
određene stranice ili članka.

Na primjer, moguće je koristiti Secure SSO za autentifikaciju korisnika, a zatim periodično provjeravati nepročitana obaveštenja i prosleđivati ih korisniku.

Pogledajte [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za primjer kako dobiti i prevesti nepročitana korisnička obaveštenja.

### Pregledač GIF-ova

Po defaultu, odabir slike ili gif-a nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podrške učitavanja slika i gif-ova. U ovoj biblioteci postoji Pregledač GIF-ova koji anonimizuje pretrage i slike koje se pružaju, potrebno ga je samo koristiti.

### Performanse

Molimo otvorite ticket sa primjerom za reprodukciju, uključujući i uređaj koji se koristi, ako uočite bilo kakve probleme sa performansama. Performanse su prvi prioritet
u svim FastComments bibliotekama.