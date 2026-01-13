### 1. Agrega FastCommentsView a tu diseño

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Inicializa y configura el SDK

```kotlin
// Configura el SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Opciones de configuración adicionales
config.voteStyle = VoteStyle.UpDown // o VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // para soporte de modo oscuro

// Inicializa el SDK
val sdk = FastCommentsSDK(config)

// Busca la vista de comentarios en tu diseño
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Asigna la instancia del SDK a la vista
commentsView.setSDK(sdk)

// Carga los comentarios
commentsView.load()
```