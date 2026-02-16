#### Skin: Erebus
![Skin: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Skin: Default
![Skin: Default](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Native WYSIWYG Editor with Image Support!
![Native WYSIWYG Editor with Image Support](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Rijke Teksteditor

Deze bibliotheek gebruikt de 10tap-editor voor rich text-bewerkingsfunctionaliteit, die een krachtige WYSIWYG-bewerkingservaring biedt.

### Configuratieopties

Deze bibliotheek streeft ernaar alle configuratie-opties te ondersteunen die zijn gedefinieerd in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), net zoals de webimplementatie.

### FastComments-concepten

De belangrijkste concepten om mee te beginnen zijn `tenantId` en `urlId`. `tenantId` is uw FastComments.com-accountidentificatie. `urlId` is waaraan commentaardraden gekoppeld zullen worden. Dit kan een pagina-URL zijn, of een product-id, een artikel-id, enz.

### Gebruikersmeldingen

FastComments ondersteunt meldingen voor [veel scenario's](https://docs.fastcomments.com/guide-notifications.html). Meldingen zijn configureerbaar, kunnen globaal of op meldings-/commentaarniveau worden uitgeschakeld, en ondersteunen abonnementen op paginaniveau zodat gebruikers zich kunnen abonneren op threads van een specifieke pagina of artikel.

Bijvoorbeeld, het is mogelijk Secure SSO te gebruiken om de gebruiker te authenticeren en vervolgens periodiek te controleren op ongelezen meldingen en deze naar de gebruiker te pushen.

Zie [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) voor hoe ongelezen gebruikersmeldingen te verkrijgen en te vertalen.

### Gif-browser

Standaard is er geen beeld- of gifselectie ingeschakeld. Zie [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) voor hoe beeld- en gifuploads te ondersteunen. Er is een Gif-browser die zoekopdrachten en afbeeldingen anonimiseert die in deze bibliotheek wordt geleverd; je hoeft deze alleen te gebruiken.

### Prestaties

Open alstublieft een ticket met een voorbeeld om te reproduceren, inclusief het gebruikte apparaat, als u prestatieproblemen constateert. Prestaties zijn een prioriteit in alle FastComments-bibliotheken.