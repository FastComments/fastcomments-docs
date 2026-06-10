#### Udseende: Erebus
![Udseende: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Udseende: Default
![Udseende: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Indbygget WYSIWYG-editor med billedunderstøttelse!
![Indbygget WYSIWYG-editor med billedunderstøttelse](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Rich Text-editor

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native, and a bundler that resolves package `exports` conditions (Metro with package exports / RN 0.72+). Web support is currently experimental.

### Konfigurationsindstillinger

Dette bibliotek har til formål at understøtte alle konfigurationsmuligheder defineret i [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), ligesom web-implementeringen.

### FastComments-konceptet

De vigtigste begreber at være opmærksom på for at komme i gang er `tenantId` og `urlId`. `tenantId` er din FastComments.com-kontoidentifikator. `urlId` er, hvor kommentartråde vil blive knyttet til. Dette kan være en side-URL, et produkt-id, en artikel-id osv.

### Brugernotifikationer

FastComments understøtter notifikationer for [mange scenarier](https://docs.fastcomments.com/guide-notifications.html). Notifikationer kan konfigureres, der kan fravælges globalt eller på notifikation/kommentar-niveau, og understøtter side-niveau abonnementer, så brugere kan abonnere på tråde for en specifik side eller artikel.

For eksempel er det muligt at bruge Secure SSO til at autentificere brugeren og derefter periodisk poll'e for ulæste notifikationer og sende dem til brugeren.

Se [eksempel AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) for, hvordan man henter og oversætter ulæste brugernotifikationer.

### Gif-browser

Som standard er der ingen billed- eller gif-udvælgelse aktiveret. Se [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) for, hvordan man understøtter billede- og gif-upload. Der er en Gif Browser, der anonymiserer søgninger og billeder leveret i dette bibliotek — du skal blot bruge den.

### Ydeevne

Åbn venligst en ticket med et eksempel til reproduktion, inklusive den anvendte enhed, hvis du identificerer eventuelle ydeevneproblemer. Ydeevne er et førsteordens hensyn i alle FastComments-biblioteker.