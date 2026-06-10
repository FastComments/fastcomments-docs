#### Skin: Erebus
![Skin: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Skin: Default
![Skin: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Native WYSIWYG-editor met afbeeldingsondersteuning!
![Native WYSIWYG Editor with Image Support](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Rijke teksteditor

Deze bibliotheek gebruikt [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) voor rijke tekstbewerking, wat een krachtige WYSIWYG-bewerkingservaring biedt. Dezelfde editor ondersteunt iOS, Android en het web (via `react-native-web`), zodat de composer zich op elk platform consequent gedraagt met één implementatie.

`react-native-enriched` vereist de React Native New Architecture (Fabric) op native niveau, en een bundler die pakket `exports`-condities oplost (Metro met package exports / RN 0.72+). Webondersteuning is momenteel experimenteel.

### Configuratie-opties

Deze bibliotheek streeft ernaar alle configuratie-opties te ondersteunen die zijn gedefinieerd in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), net als de webimplementatie.

### FastComments-concepten

De belangrijkste concepten om te weten om aan de slag te gaan zijn `tenantId` en `urlId`. `tenantId` is uw FastComments.com-accountidentificatie. `urlId` is waaraan commentaardraden gekoppeld zullen worden. Dit kan een paginapadre, een product-id, een artikel-id, enz. zijn.

### Gebruikersmeldingen

FastComments ondersteunt meldingen voor [veel scenario's](https://docs.fastcomments.com/guide-notifications.html). Meldingen zijn configureerbaar, kunnen globaal of op meldings/commentaarniveau worden uitgeschakeld, en ondersteunen paginaniveau-abonnementen zodat gebruikers zich kunnen abonneren op discussiedraden van een specifieke pagina of artikel.

Het is bijvoorbeeld mogelijk Secure SSO te gebruiken om de gebruiker te authenticeren en vervolgens periodiek te polleren op ongelezen meldingen en deze naar de gebruiker te pushen.

Zie [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) voor hoe ongelezen gebruikersmeldingen te verkrijgen en te vertalen.

### Gif-browser

Standaard is geen afbeeldings- of gif-selectie ingeschakeld. Zie [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) voor hoe ondersteuning voor afbeelding- en gif-uploads toe te voegen. Er is een Gif-browser die zoekopdrachten en afbeeldingen anonimiseert die in deze bibliotheek wordt meegeleverd, u hoeft deze alleen maar te gebruiken.

### Prestaties

Open alstublieft een ticket met een voorbeeld om te reproduceren, inclusief gebruikt apparaat, als u prestatieproblemen ontdekt. Prestaties zijn een topprioriteit in alle FastComments-bibliotheken.