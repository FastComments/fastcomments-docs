Los métodos del SDK lanzan `FastCommentsError`, que cumple con `LocalizedError`:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

Propiedades de `FastCommentsError`:

- `code` -- código de error de la API
- `reason` -- descripción del error en inglés
- `translatedError` -- mensaje de error localizado proporcionado por el servidor

Los errores de bloqueo también se muestran automáticamente a través de `sdk.blockingErrorMessage`, que las vistas integradas muestran al usuario.

---
---