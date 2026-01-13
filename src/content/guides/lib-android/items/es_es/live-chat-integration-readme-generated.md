Agrega una interfaz de chat en tiempo real a tu aplicación:

```kotlin
// Agrega LiveChatView a tu layout XML
// <com.fastcomments.sdk.LiveChatView
//     android:id="@+id/liveChatView"
//     android:layout_width="match_parent"
//     android:layout_height="match_parent" />

// Crea una configuración para el SDK
val config = CommentWidgetConfig().apply {
    tenantId = "your-tenant-id"
    urlId = "chat-room-identifier" 
    pageTitle = "Chat Room Name"
}
LiveChatView.setupLiveChatConfig(config)

// Opcional: Agrega autenticación de usuario
val userData = SimpleSSOUserData(
    "User Name",
    "user@example.com",
    "https://path-to-avatar.jpg"
)
val sso = FastCommentsSSO(userData)
config.sso = sso.prepareToSend()

// Inicializa el SDK
val sdk = FastCommentsSDK().configure(config)

// Configura la vista de chat en vivo
val liveChatView = findViewById<LiveChatView>(R.id.liveChatView)
liveChatView.setSDK(sdk)
liveChatView.load()

// No olvides el manejo del ciclo de vida
override fun onResume() {
    super.onResume()
    sdk.refreshLiveEvents()
}

override fun onDestroy() {
    super.onDestroy()
    sdk.cleanup()
}
```
---