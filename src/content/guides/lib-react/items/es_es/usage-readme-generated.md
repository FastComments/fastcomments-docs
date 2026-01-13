### El componente principal del widget

El componente FastCommentsCommentWidget contiene el widget de comentarios en vivo de FastComments.

Reemplace "demo" abajo con su "tenantId" - disponible [aquí](https://fastcomments.com/auth/my-account/api) en el área de administración de FastComments.

El widget admite muchas opciones - consulte FastCommentsCommentWidgetConfig en src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Actualizar la página actual (para SPAs)
Para actualizar la página/artículo al que está vinculada la conversación de comentarios, debe actualizar los parámetros de configuración "urlId" y "url".
Vea el ejemplo y la explicación [aquí](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Región de la cuenta (ATENCIÓN: clientes de la UE)

Si está en la UE, querrá indicar a los widgets del cliente en qué región se encuentra. Vea [examples/example-eu](/examples/example-eu/src/App.tsx);
De lo contrario, no tiene que definir `region`.

### El widget de recuento de comentarios

El componente FastCommentsCommentCountWidget contiene el widget de recuento de comentarios en vivo de FastComments.

Reemplace "demo" abajo con su "tenantId" - disponible [aquí](https://fastcomments.com/auth/my-account/api) en el área de administración de FastComments.

Consulte FastCommentsCommentCountConfig en src/index.tsx para ver las opciones de configuración compatibles.

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

Para una implementación completamente nativa de FastComments, consulte [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Para un envoltorio de React Native de esta biblioteca, que utiliza un webview, consulte [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).