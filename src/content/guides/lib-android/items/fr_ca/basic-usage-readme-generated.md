### 1. Ajouter FastCommentsView à votre mise en page

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Initialiser et configurer le SDK

```kotlin
// Configurer le SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Options de configuration supplémentaires
config.voteStyle = VoteStyle.UpDown // ou VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // pour la prise en charge du mode sombre

// Initialiser le SDK
val sdk = FastCommentsSDK(config)

// Trouver la vue des commentaires dans votre mise en page
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Associer l'instance du SDK à la vue
commentsView.setSDK(sdk)

// Charger les commentaires
commentsView.load()
```