### The Main Widget Component

Il componente FastCommentsCommentWidget contiene il widget dei commenti FastComments in tempo reale.

Sostituisci "demo" qui sotto con il tuo "tenantId" - disponibile [qui](https://fastcomments.com/auth/my-account/api) nell'area di amministrazione di FastComments.

Il widget supporta molte opzioni - vedere FastCommentsCommentWidgetConfig in src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Updating The Current Page (For SPAs)
Per aggiornare la pagina/articolo a cui è associato il thread dei commenti è necessario aggiornare i parametri di configurazione "urlId" e "url".
Vedi l'esempio e la spiegazione [qui](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Account Region (ATTENTION: EU Customers)

Se ti trovi nell'UE, vorrai indicare ai widget client in quale regione ti trovi. Vedi [examples/example-eu](/examples/example-eu/src/App.tsx);
Altrimenti, non è necessario definire `region`.

### The Comment Count Widget

Il componente FastCommentsCommentCountWidget contiene il widget del conteggio dei commenti FastComments in tempo reale.

Sostituisci "demo" qui sotto con il tuo "tenantId" - disponibile [qui](https://fastcomments.com/auth/my-account/api) nell'area di amministrazione di FastComments.

Vedi FastCommentsCommentCountConfig in src/index.tsx per le opzioni di configurazione supportate.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Native

Per un'implementazione completamente nativa di FastComments, vedi [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Per un wrapper React Native di questa libreria, che utilizza una webview, vedi [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).