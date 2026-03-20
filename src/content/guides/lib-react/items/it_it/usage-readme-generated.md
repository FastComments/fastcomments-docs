### Il componente principale del widget

Il componente FastCommentsCommentWidget contiene il widget dei commenti FastComments in tempo reale.

Sostituisci "demo" qui sotto con il tuo "tenantId" - disponibile [here](https://fastcomments.com/auth/my-account/api) nell'area di amministrazione FastComments.

Il widget supporta molte opzioni - vedi FastCommentsCommentWidgetConfig in src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Aggiornamento della pagina corrente (per le SPA)
Per aggiornare la pagina/articolo a cui è legato il thread dei commenti, devi aggiornare i parametri di configurazione "urlId" e "url".
Vedi l'esempio e la spiegazione [here](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Regione dell'account (ATTENZIONE: clienti UE)

Se ti trovi nell'UE, dovrai indicare ai widget client in quale regione ti trovi. Vedi [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Altrimenti, non è necessario definire `region`.

### Il widget del conteggio dei commenti

Il componente FastCommentsCommentCountWidget contiene il widget di conteggio dei commenti FastComments in tempo reale.

Sostituisci "demo" qui sotto con il tuo "tenantId" - disponibile [here](https://fastcomments.com/auth/my-account/api) nell'area di amministrazione FastComments.

Consulta FastCommentsCommentCountConfig in src/index.tsx per le opzioni di configurazione supportate.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Nativo

Per un'implementazione completamente nativa di FastComments, vedi [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Per un wrapper React Native di questa libreria, che utilizza una webview, vedi [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).