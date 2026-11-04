### Tamanho da Página

```swift
// Comments: padrão 30
sdk.pageSize = 50

// Feed: padrão 10
feedSDK.pageSize = 20
```

### Carregando Mais Comentários

A UI mostra os controles de paginação automaticamente. Você também pode acionar a paginação programaticamente:

```swift
// Carregar próxima página
try await sdk.loadMore()

// Carregar todos os restantes (desativado se >2000 comentários por questões de desempenho)
try await sdk.loadAll()

// Verificar estado
sdk.hasMore            // Se existem mais páginas
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginação de Comentários Filhos

As respostas aninhadas são carregadas de forma preguiçosa (lazy). Quando um usuário expande uma thread, os primeiros 5 filhos são carregados. Um controle de "carregar mais respostas" aparece se houver mais. Isso é tratado automaticamente pela UI.

---