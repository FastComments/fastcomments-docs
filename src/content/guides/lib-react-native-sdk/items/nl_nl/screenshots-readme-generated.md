#### Skin: Erebus
![Skin: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Skin: Default
![Skin: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Native WYSIWYG Editor met afbeeldingsondersteuning!
![Native WYSIWYG Editor with Image Support](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Rich Text Editor

Deze bibliotheek gebruikt de 10tap editor voor rich text-bewerkingsfunctionaliteit, die een krachtige WYSIWYG-bewerkingservaring biedt.

### Configuration Options

Deze bibliotheek streeft ernaar alle configuratie-opties te ondersteunen die zijn gedefinieerd in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), net als de webimplementatie.

### FastComments Concepts

De belangrijkste concepten om mee te beginnen zijn `tenantId` en `urlId`. `tenantId` is uw FastComments.com accountidentificatie. `urlId` is waaraan commentaardraden gekoppeld zullen worden. Dit kan een pagina-URL zijn, of een product-id, een artikel-id, enz.

### User Notifications

FastComments ondersteunt meldingen voor [veel scenario's](https://docs.fastcomments.com/guide-notifications.html). Meldingen zijn configureerbaar,
kunnen globaal of op meldings-/commentaarniveau worden uitgeschakeld, en ondersteunen paginaniveau-abonnementen zodat gebruikers zich op draden van een
specifieke pagina of artikel kunnen abonneren.

Bijvoorbeeld, het is mogelijk om Beveiligde SSO te gebruiken om de gebruiker te authenticeren en vervolgens periodiek te controleren op ongelezen meldingen en deze naar de gebruiker te pushen.

Zie [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) voor hoe je ongelezen gebruikersmeldingen kunt ophalen en vertalen.

### Gif Browser

Standaard is geen selectie van afbeeldingen of gifs ingeschakeld. Zie [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) voor hoe
je afbeelding- en gif-uploads kunt ondersteunen. Er is een Gif Browser die zoekopdrachten en afbeeldingen anonimiseert die in deze bibliotheek worden geleverd; je hoeft deze alleen maar te gebruiken.

### Performance

Open alstublieft een ticket met een voorbeeld om te reproduceren, inclusief het gebruikte apparaat, als je prestatieproblemen ontdekt. Prestaties zijn een topprioriteit
in alle FastComments-bibliotheken.