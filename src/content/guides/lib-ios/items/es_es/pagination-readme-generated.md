### Tamaño de página

```swift
// Comentarios: por defecto 30
sdk.pageSize = 50

// Feed: por defecto 10
feedSDK.pageSize = 20
```

### Cargar más comentarios

La interfaz de usuario muestra controles de paginación automáticamente. También puedes activar la paginación programáticamente:

```swift
// Cargar la siguiente página
try await sdk.loadMore()

// Cargar todos los restantes (deshabilitado si >2000 comentarios por rendimiento)
try await sdk.loadAll()

// Comprobar estado
sdk.hasMore            // Si existen más páginas
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginación de comentarios hijos

Las respuestas anidadas se cargan de forma diferida. Cuando un usuario expande un hilo, se cargan los primeros 5 hijos. Aparece un control "cargar más respuestas" si existen más. Esto lo gestiona automáticamente la interfaz de usuario.

---
---