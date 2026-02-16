#### Udseende: Erebus
![Udseende: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Udseende: Standard
![Udseende: Standard](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Indbygget WYSIWYG-editor med billedunderstøttelse!
![Indbygget WYSIWYG-editor med billedunderstøttelse](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Rich Text-editor

Dette bibliotek bruger 10tap-editoren til rich text-redigering, hvilket giver en kraftfuld WYSIWYG-redigeringsoplevelse.

### Konfigurationsmuligheder

Dette bibliotek har til mål at understøtte alle konfigurationsmuligheder defineret i [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), præcis som webimplementeringen.

### FastComments-koncepter

De vigtigste begreber at være opmærksom på for at komme i gang er `tenantId` og `urlId`. `tenantId` er din FastComments.com-kontoidentifikator. `urlId` er, hvor kommentartråde vil blive knyttet til. Dette kan være en side-URL, eller et produkt-id, et artikel-id osv.

### Brugerunderretninger

FastComments understøtter notifikationer for [mange scenarier](https://docs.fastcomments.com/guide-notifications.html). Notifikationer kan konfigureres, fravælges globalt eller på notifikation-/kommentarniveau, og understøtter abonnementer på sideniveau, så brugere kan abonnere på tråde for en bestemt side eller artikel.

For eksempel er det muligt at bruge Secure SSO til at godkende brugeren og derefter periodisk forespørge efter ulæste notifikationer og sende dem til brugeren.

Se [eksemplet AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for hvordan man henter og oversætter ulæste brugerunderretninger.

### Gif-browser

Som standard er der ingen billed- eller gif-selektion aktiveret. Se [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for hvordan man understøtter billed- og gif-upload. Der er en Gif Browser, der anonymiserer søgninger og billeder leveret i dette bibliotek — du skal blot bruge den.

### Ydeevne

Åbn venligst en ticket med et eksempel for at reproducere problemet, inklusive anvendt enhed, hvis du identificerer ydeevneproblemer. Ydeevne er en topprioritet i alle FastComments-biblioteker.