#### Videz: Erebus
![Videz: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Videz: Privzeto
![Videz: Privzeto](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Vgrajen WYSIWYG urejevalnik s podporo za slike!
![Vgrajen WYSIWYG urejevalnik s podporo za slike](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Urejevalnik obogatenega besedila

Ta knjižnica uporablja urejevalnik 10tap za funkcionalnost urejanja obogatenega besedila, ki zagotavlja zmogljivo WYSIWYG izkušnjo urejanja.

### Možnosti konfiguracije

Ta knjižnica si prizadeva podpirati vse možnosti konfiguracije, opredeljene v [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), enako kot spletna implementacija.

### Koncepti FastComments

Glavni pojmi, ki jih je treba poznati za začetek, so `tenantId` in `urlId`. `tenantId` je identifikator vašega računa na FastComments.com. `urlId` je mesto, kjer bodo pripete niti komentarjev. To je lahko URL strani, ID izdelka, ID članka itd.

### Uporabniška obvestila

FastComments podpira obvestila za [veliko scenarijev](https://docs.fastcomments.com/guide-notifications.html). Obvestila so nastavljiva, se jim je mogoče odjaviti globalno ali na ravni posameznega obvestila/komentarja, in podpirajo naročnine na ravni strani, tako da se lahko uporabniki naročijo na niti na določeni strani ali članku.

Na primer, mogoče je uporabiti Secure SSO za preverjanje pristnosti uporabnika in nato periodično preverjati neprebrana obvestila ter jih posredovati uporabniku.

Oglejte si [primer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za to, kako pridobiti in prevesti neprebrana uporabniška obvestila.

### Brskalnik GIF-ov

Privzeto izbira slik ali GIF-ov ni omogočena. Oglejte si [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za to, kako podpreti nalaganje slik in GIF-ov. V tej knjižnici je na voljo brskalnik GIF-ov, ki anonimizira iskanja in slike, ki jih zagotavlja — morate ga le uporabiti.

### Zmogljivost

Če opazite težave z zmogljivostjo, prosimo odprite prijavo (ticket) z primerom, kako jo reproducirati, vključno z uporabljeno napravo. Zmogljivost je prednostna naloga v vseh knjižnicah FastComments.