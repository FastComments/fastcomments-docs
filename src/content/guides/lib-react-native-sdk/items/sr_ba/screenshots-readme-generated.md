#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Default
![Tema: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Nativni WYSIWYG uređivač sa podrškom za slike!
![Nativni WYSIWYG uređivač sa podrškom za slike](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Uređivač bogatog teksta

Ova biblioteka koristi 10tap editor za funkcionalnost uređivanja bogatog teksta, koji pruža moćno WYSIWYG iskustvo uređivanja.

### Opcije konfiguracije

Ova biblioteka ima za cilj podržati sve opcije konfiguracije definirane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), isto kao i web implementacija.

### Koncepti FastComments

Glavni koncepti koje treba znati za početak su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` označava za šta će biti vezane niti komentara. To može biti URL stranice, ili id proizvoda, id članka, itd.

### Obavještenja korisnika

FastComments podržava obavještenja za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obavještenja su konfigurabilna, korisnik se može odjaviti globalno ili na nivou pojedinačnog obavještenja/komentara, i podržava pretplate na nivou stranice tako da se korisnici mogu pretplatiti na niti komentara određene stranice ili članka.

Na primjer, moguće je koristiti Secure SSO za autentifikaciju korisnika i zatim periodično provjeravati nepročitana obavještenja i slati ih korisniku.

Pogledajte [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način kako dobiti i prevesti nepročitana obavještenja korisnika.

### Pregledač GIF-ova

Po defaultu nije omogućeno odabiranje slika ili GIF-ova. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) kako podržati upload slika i GIF-ova. U ovoj biblioteci postoji Pregledač GIF-ova koji anonimizira pretrage i slike, samo ga trebate koristiti.

### Performanse

Molimo otvorite ticket sa primjerom za reprodukciju, uključujući uređaj koji ste koristili, ako primijetite bilo kakve probleme s performansama. Performanse su prioritet u svim FastComments bibliotekama.