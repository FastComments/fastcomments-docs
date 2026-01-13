### The Main Widget Component

O componente FastCommentsCommentWidget contém o widget de comentários do FastComments em tempo real.

Substitua "demo" abaixo pelo seu "tenantId" - disponível [aqui](https://fastcomments.com/auth/my-account/api) na área administrativa do FastComments.

O widget suporta muitas opções - veja FastCommentsCommentWidgetConfig em src/index.tsx.

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
Para atualizar a página/artigo ao qual o tópico de comentários está vinculado, você deve atualizar os parâmetros de configuração "urlId" e "url".
Veja o exemplo e a explicação [aqui](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Account Region (ATTENTION: EU Customers)

Se você está na UE, você deve informar aos widgets do cliente em qual região você está. Veja [examples/example-eu](/examples/example-eu/src/App.tsx);
Caso contrário, você não precisa definir `region`.

### The Comment Count Widget

O componente FastCommentsCommentCountWidget contém o widget de contagem de comentários do FastComments em tempo real.

Substitua "demo" abaixo pelo seu "tenantId" - disponível [aqui](https://fastcomments.com/auth/my-account/api) na área administrativa do FastComments.

Veja FastCommentsCommentCountConfig em src/index.tsx para as opções de configuração suportadas.

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

Para uma implementação completamente nativa do FastComments, veja [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Para um wrapper React Native desta biblioteca, usando um webview, veja [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).