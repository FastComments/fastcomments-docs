### Le composant principal du widget

Le composant FastCommentsCommentWidget contient le widget de commentaires en direct de FastComments.

Remplacez "demo" ci-dessous par votre "tenantId" - disponible [ici](https://fastcomments.com/auth/my-account/api) dans la zone d'administration de FastComments.

Le widget offre de nombreuses options - voir FastCommentsCommentWidgetConfig dans src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Mise à jour de la page courante (pour les SPA)
Pour mettre à jour la page/l'article auquel le fil de commentaires est associé, vous devez mettre à jour les paramètres de configuration "urlId" et "url".
Voir l'exemple et les explications [ici](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Région du compte (ATTENTION : clients de l'UE)

Si vous êtes dans l'UE, vous devrez indiquer la région où vous vous trouvez aux widgets clients. Voir [examples/example-eu](/examples/example-eu/src/App.tsx);
Sinon, vous n'avez pas à définir `region`.

### Le widget du compteur de commentaires

Le composant FastCommentsCommentCountWidget contient le widget de compteur de commentaires en direct de FastComments.

Remplacez "demo" ci-dessous par votre "tenantId" - disponible [ici](https://fastcomments.com/auth/my-account/api) dans la zone d'administration de FastComments.

Voir FastCommentsCommentCountConfig dans src/index.tsx pour les options de configuration prises en charge.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Natif

Pour une implémentation complètement native de FastComments, voir [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Pour un wrapper React Native de cette bibliothèque, utilisant une webview, voir [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).