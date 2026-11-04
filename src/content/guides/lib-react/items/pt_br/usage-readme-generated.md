### O Componente Principal do Widget

O componente FastCommentsCommentWidget contém o widget de comentários ao vivo do FastComments.

Substitua "demo" abaixo pelo seu "tenantId" - disponível [aqui](https://fastcomments.com/auth/my-account/api) na área de administração do FastComments.

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

### Atualizando a Página Atual (Para SPAs)
Para atualizar a página/artigo ao qual o tópico de comentários está vinculado, você deve atualizar os parâmetros de configuração "urlId" e "url".
Veja o exemplo e a explicação [aqui](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Região da Conta (ATENÇÃO: Clientes da UE)

Se você estiver na UE, será necessário informar aos widgets do cliente em qual região você está. Veja [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Caso contrário, você não precisa definir `region`.

### O Widget de Contagem de Comentários

O componente FastCommentsCommentCountWidget contém o widget de contagem de comentários ao vivo do FastComments.

Substitua "demo" abaixo pelo seu "tenantId" - disponível [aqui](https://fastcomments.com/auth/my-account/api) na área de administração do FastComments.

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

### Nativo

Para uma implementação totalmente nativa do FastComments, veja [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Para um wrapper React Native desta biblioteca, usando uma webview, veja [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).