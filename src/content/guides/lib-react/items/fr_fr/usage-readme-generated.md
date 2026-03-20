### Le composant principal du widget

Le composant FastCommentsCommentWidget contient le widget de commentaires FastComments en direct.

Remplacez "demo" ci‑dessous par votre "tenantId" - disponible [ici](https://fastcomments.com/auth/my-account/api) dans la zone d'administration FastComments.

Le widget prend en charge de nombreuses options - voir FastCommentsCommentWidgetConfig dans src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Mettre à jour la page courante (pour les SPAs)
Pour mettre à jour la page/article auquel le fil de commentaires est lié, vous devez mettre à jour les paramètres de configuration "urlId" et "url".
Voir l'exemple et l'explication [ici](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Région du compte (ATTENTION : clients UE)

Si vous êtes dans l'UE, vous devrez indiquer aux widgets clients dans quelle région vous vous trouvez. Voir [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Sinon, vous n'avez pas à définir `region`.

### Le widget de comptage des commentaires

Le composant FastCommentsCommentCountWidget contient le widget de comptage des commentaires FastComments en direct.

Remplacez "demo" ci‑dessous par votre "tenantId" - disponible [ici](https://fastcomments.com/auth/my-account/api) dans la zone d'administration FastComments.

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

Pour une implémentation entièrement native de FastComments, voir [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Pour un wrapper React Native de cette bibliothèque, utilisant une webview, voir [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).