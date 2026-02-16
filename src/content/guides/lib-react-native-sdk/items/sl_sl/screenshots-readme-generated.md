#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Default
![Tema: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Nativen WYSIWYG urejevalnik s podporo za slike!
![Nativen WYSIWYG urejevalnik s podporo za slike](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Urejevalnik bogatega besedila

Ta knjižnica uporablja urejevalnik 10tap za funkcionalnost urejanja bogatega besedila, ki zagotavlja zmogljivo WYSIWYG izkušnjo urejanja.

### Možnosti konfiguracije

Ta knjižnica si prizadeva podpreti vse možnosti konfiguracije, definirane v [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), enako kot spletna implementacija.

### Koncepti FastComments

Glavna koncepta, ki ju morate poznati za začetek, sta `tenantId` in `urlId`. `tenantId` je identifier vašega računa FastComments.com. `urlId` je mesto, kjer bodo pripeti niti komentarjev. To je lahko URL strani, id izdelka, id članka itd.

### Uporabniška obvestila

FastComments podpira obvestila za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obvestila so konfigurabilna, uporabniki se lahko odjavijo globalno ali na ravni obvestila/komentarja in podpirajo naročnine na ravni strani, tako da se lahko uporabniki naročijo na niti določene strani ali članka.

Na primer, mogoče je uporabiti Secure SSO za overjanje uporabnika in nato periodično preverjati neprebrana obvestila ter jih poslati uporabniku.

Oglejte si [primer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx), kako pridobiti in prevesti neprebrana uporabniška obvestila.

### Brskalnik GIF-ov

Privzeto ni omogočena izbira slik ali gifov. Oglejte si [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx), kako podpreti nalaganje slik in gifov. V tej knjižnici je na voljo Brskalnik GIF-ov, ki anonimizira iskanja in slike, ki jih ponuja, morate ga le uporabiti.

### Zmogljivost

Prosimo, odprite prijavo (ticket) z vzorcem za ponovitev, vključno z napravo, če opazite kakršne koli težave z zmogljivostjo. Zmogljivost je pri vseh knjižnicah FastComments prednostna naloga.