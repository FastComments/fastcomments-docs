#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Default
![Tema: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Nativni WYSIWYG editor sa podrškom za slike!
![Nativni WYSIWYG editor sa podrškom za slike](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Editor za bogat tekst

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za uređivanje bogatog teksta, što obezbeđuje moćno WYSIWYG iskustvo uređivanja. Isti editor pokreće iOS, Android i web (putem `react-native-web`), tako da se uređivač ponaša konzistentno na svim platformama uz jednu implementaciju.

`react-native-enriched` zahteva React Native New Architecture (Fabric) na nativnim platformama, i bundler koji rešava uslove `exports` paketa (Metro sa package exports / RN 0.72+). Podrška za web je trenutno eksperimentalna.

### Opcije konfiguracije

Ova biblioteka ima za cilj da podrži sve opcije konfiguracije definisane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), isto kao i web implementacija.

### Koncepti FastComments

Glavni koncepti koje treba znati za početak su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` označava za šta će biti vezane niti komentara. To može biti URL stranice, id proizvoda, id članka itd.

### Korisničke obaveštenja

FastComments podržava obaveštenja za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obaveštenja se mogu konfigurisati, moguće je odjaviti se globalno ili na nivou pojedinačnog obaveštenja/komentara, i podržane su pretplate po stranici tako da se korisnici mogu pretplatiti na niti određene stranice ili članka.

Na primer, moguće je koristiti Secure SSO za autentifikaciju korisnika, a zatim periodično proveravati nepročitana obaveštenja i slati ih korisniku.

Pogledajte [primer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za to kako dobiti i prevesti nepročitana korisnička obaveštenja.

### Pregledač GIF-ova

Po defaultu, izbor slika ili gifova nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) kako podržati otpremanje slika i gifova. Postoji Pregledač GIF-ova koji anonimizuje pretrage i slike dostavljene u ovoj biblioteci — potrebno je samo da ga koristite.

### Performanse

Ako primetite probleme sa performansama, otvorite tiket sa primerom za reprodukciju, uključujući uređaj koji je korišćen. Performanse su prioritet u svim FastComments bibliotekama.