### 1. Adicione o FastCommentsView ao seu layout

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Inicialize e configure o SDK

```kotlin
// Configure o SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Opções adicionais de configuração
config.voteStyle = VoteStyle.UpDown // ou VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // para suporte ao modo escuro

// Inicialize o SDK
val sdk = FastCommentsSDK(config)

// Encontre a view de comentários no seu layout
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Defina a instância do SDK para a view
commentsView.setSDK(sdk)

// Carregue os comentários
commentsView.load()
```