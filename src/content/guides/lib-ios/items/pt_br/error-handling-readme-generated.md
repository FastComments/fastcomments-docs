---
Os métodos do SDK lançam `FastCommentsError`, que está em conformidade com `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

Propriedades de `FastCommentsError`:

- `code` -- código de erro da API
- `reason` -- descrição do erro em inglês
- `translatedError` -- mensagem de erro localizada fornecida pelo servidor

Erros de bloqueio também são exibidos automaticamente via `sdk.blockingErrorMessage`, que as visualizações integradas exibem para o usuário.

---
---