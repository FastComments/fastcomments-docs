#### Tema: Erebus
![Tema: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Tema: Default
![Tema: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Indbygget WYSIWYG-editor med billedunderstøttelse!
![Indbygget WYSIWYG-editor med billedunderstøttelse](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Rich Text Editor

Dette bibliotek bruger 10tap-editoren til rig tekstredigering, hvilket giver en kraftfuld WYSIWYG-redigeringsoplevelse.

### Konfigurationsmuligheder

Dette bibliotek har til formål at understøtte alle konfigurationsmuligheder defineret i [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), præcis som webimplementeringen.

### FastComments-koncepter

De vigtigste koncepter at være opmærksom på for at komme i gang er `tenantId` og `urlId`. `tenantId` er dit FastComments.com-konto-id. `urlId` er, hvor kommentartråde
vil blive knyttet til. Dette kan være en side-URL, et produkt-id, et artikel-id osv.

### Brugernotifikationer

FastComments understøtter notifikationer for [mange scenarier](https://docs.fastcomments.com/guide-notifications.html). Notifikationer kan konfigureres,
fravælges globalt eller på notifikations-/kommentarniveau og understøtter side-abonnementer, så brugere kan abonnere på tråde for en
specifik side eller artikel.

For eksempel er det muligt at bruge Secure SSO til at autentificere brugeren og derefter periodisk forespørge efter ulæste notifikationer og skubbe dem til brugeren.

Se [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for, hvordan du henter og oversætter ulæste brugernotifikationer.

### GIF-browser

Som standard er ingen billed- eller GIF-udvælgelse aktiveret. Se [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for, hvordan
du understøtter billede- og GIF-upload. Der er en GIF-browser i dette bibliotek, som anonymiserer søgninger og billeder, du skal blot bruge den.

### Ydeevne

Åbn venligst en ticket med et eksempel, der kan genskabe problemet, inklusive den anvendte enhed, hvis du identificerer ydeevneproblemer. Ydeevne er en topprioritet
i alle FastComments-biblioteker.