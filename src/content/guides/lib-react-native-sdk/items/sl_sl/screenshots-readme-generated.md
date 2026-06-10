#### Slog: Erebus
![Slog: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Slog: Privzeto
![Slog: Privzeto](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Nativni WYSIWYG urejevalnik z podporo slik!
![Nativni WYSIWYG urejevalnik s podporo slik](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Urejevalnik bogatega besedila

Ta knjižnica uporablja [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za urejanje bogatega besedila, kar zagotavlja zmogljivo WYSIWYG izkušnjo urejanja. Isti urejevalnik poganja iOS, Android in splet (prek `react-native-web`), zato se komponist obnaša dosledno na vseh platformah z eno samo implementacijo.

`react-native-enriched` zahteva React Native New Architecture (Fabric) na nativni strani in bundler, ki rešuje pogoje `exports` paketov (Metro z package exports / RN 0.72+). Podpora za splet je trenutno eksperimentalna.

### Možnosti konfiguracije

Ta knjižnica si prizadeva podpirati vse možnosti konfiguracije, opredeljene v [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), enako kot spletna implementacija.

### Koncepti FastComments

Glavna koncepta, ki ju morate poznati za začetek, sta `tenantId` in `urlId`. `tenantId` je identifikator vašega računa na FastComments.com. `urlId` označuje, kje bodo pripete nitke komentarjev. To je lahko URL strani, id izdelka, id članka itd.

### Obvestila uporabnikov

FastComments podpira obvestila za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obvestila so konfigurabilna, uporabniki se lahko odjavijo globalno ali na ravni posameznega obvestila/komentarja, podprte so tudi naročnine na ravni strani, tako da se lahko uporabniki naročijo na nitke določenega članka ali strani.

Na primer, možno je uporabiti Secure SSO za overjanje uporabnika in nato periodično preverjati neprebrana obvestila ter jih potisniti do uporabnika.

Oglejte si [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za primer, kako pridobiti in prevesti neprebrana uporabniška obvestila.

### Brskalnik GIF-ov

Privzeto ni omogočena nobena izbira slik ali gifov. Oglejte si [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podpore nalaganja slik in gifov. V tej knjižnici je na voljo Brskalnik GIF-ov, ki anonimizira iskanja in slike, preprosto ga morate uporabiti.

### Zmogljivost

Če opazite kakršnekoli težave z zmogljivostjo, odprite ticket z primerom za reproduciranje, vključno z napravo, ki ste jo uporabili. Zmogljivost je ena glavnih prioritet vseh knjižnic FastComments.