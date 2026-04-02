Tanto `FastCommentsSDK` quanto `FastCommentsFeedSDK` são classes `ObservableObject` com propriedades `@Published`. Você pode observar essas propriedades nas suas views SwiftUI para atualizações reativas da UI.

### Propriedades Publicadas do FastCommentsSDK

| Propriedade | Tipo | Descrição |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Contagem total de comentários no servidor |
| `newRootCommentCount` | `Int` | Comentários novos em buffer (quando `showLiveRightAway` é false) |
| `currentUser` | `UserSessionInfo?` | Usuário autenticado atual |
| `isSiteAdmin` | `Bool` | Se o usuário atual é administrador do site |
| `isClosed` | `Bool` | Se o tópico de comentários está fechado |
| `hasBillingIssue` | `Bool` | Se há um problema de cobrança |
| `isLoading` | `Bool` | Se uma requisição de rede está em andamento |
| `hasMore` | `Bool` | Se existem mais páginas de comentários |
| `blockingErrorMessage` | `String?` | Erro que impede o funcionamento da UI |
| `warningMessage` | `String?` | Mensagem de aviso não bloqueante |
| `isDemo` | `Bool` | Se está em modo de demonstração |
| `commentsVisible` | `Bool` | Alterna a visibilidade dos comentários |
| `toolbarEnabled` | `Bool` | Se a barra de ferramentas de formatação está exibida |

### Propriedades Publicadas do FastCommentsFeedSDK

| Propriedade | Tipo | Descrição |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Posts do feed carregados atualmente |
| `hasMore` | `Bool` | Se existem mais páginas |
| `currentUser` | `UserSessionInfo?` | Usuário autenticado atual |
| `blockingErrorMessage` | `String?` | Mensagem de erro bloqueante |
| `isLoading` | `Bool` | Se uma requisição de rede está em andamento |
| `newPostsCount` | `Int` | Número de novos posts desde o último carregamento |

### Árvore de Comentários

A árvore de comentários é acessível via `sdk.commentsTree`:

```swift
// Lista plana de nós visíveis para renderização
sdk.commentsTree.visibleNodes

// Localiza um comentário pelo ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---