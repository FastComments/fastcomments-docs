Fügen Sie Ihrer App eine Echtzeit-Chat-Schnittstelle hinzu:

```kotlin
// Fügen Sie LiveChatView zu Ihrem Layout-XML hinzu
// <com.fastcomments.sdk.LiveChatView
//     android:id="@+id/liveChatView"
//     android:layout_width="match_parent"
//     android:layout_height="match_parent" />

// Erstellen Sie eine Konfiguration für das SDK
val config = CommentWidgetConfig().apply {
    tenantId = "your-tenant-id"
    urlId = "chat-room-identifier" 
    pageTitle = "Chat Room Name"
}
LiveChatView.setupLiveChatConfig(config)

// Optional: Benutzer-Authentifizierung hinzufügen
val userData = SimpleSSOUserData(
    "User Name",
    "user@example.com",
    "https://path-to-avatar.jpg"
)
val sso = FastCommentsSSO(userData)
config.sso = sso.prepareToSend()

// Initialisieren Sie das SDK
val sdk = FastCommentsSDK().configure(config)

// Richten Sie die LiveChatView ein
val liveChatView = findViewById<LiveChatView>(R.id.liveChatView)
liveChatView.setSDK(sdk)
liveChatView.load()

// Vergessen Sie nicht die Lebenszyklusbehandlung
override fun onResume() {
    super.onResume()
    sdk.refreshLiveEvents()
}

override fun onDestroy() {
    super.onDestroy()
    sdk.cleanup()
}
```