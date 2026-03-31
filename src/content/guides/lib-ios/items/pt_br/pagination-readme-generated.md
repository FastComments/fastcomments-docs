### Page Size

```swift
// Comentários: padrão 30
sdk.pageSize = 50

// Feed: padrão 10
feedSDK.pageSize = 20
```

### Loading More Comments

A IU exibe controles de paginação automaticamente. Você também pode acionar a paginação programaticamente:

```swift
// Carregar próxima página
try await sdk.loadMore()

// Carregar todos os restantes (desativado se >2000 comentários por motivos de desempenho)
try await sdk.loadAll()

// Verificar estado
sdk.hasMore            // Se há mais páginas
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Child Comment Pagination

As respostas aninhadas são carregadas sob demanda. Quando um usuário expande um tópico, os primeiros 5 comentários filhos são carregados. Um controle "carregar mais respostas" aparece se houver mais. Isso é tratado automaticamente pela IU.