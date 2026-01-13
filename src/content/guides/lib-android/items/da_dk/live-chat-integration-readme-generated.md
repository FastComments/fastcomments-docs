Tilføj en realtids-chatgrænseflade til din app:

```kotlin
// Tilføj LiveChatView til din layout-XML
// <com.fastcomments.sdk.LiveChatView
//     android:id="@+id/liveChatView"
//     android:layout_width="match_parent"
//     android:layout_height="match_parent" />

// Opret en konfiguration for SDK'en
val config = CommentWidgetConfig().apply {
    tenantId = "your-tenant-id"
    urlId = "chat-room-identifier" 
    pageTitle = "Chat Room Name"
}
LiveChatView.setupLiveChatConfig(config)

// Valgfrit: Tilføj brugerautentificering
val userData = SimpleSSOUserData(
    "User Name",
    "user@example.com",
    "https://path-to-avatar.jpg"
)
val sso = FastCommentsSSO(userData)
config.sso = sso.prepareToSend()

// Initialiser SDK'en
val sdk = FastCommentsSDK().configure(config)

// Opsæt live chat-visningen
val liveChatView = findViewById<LiveChatView>(R.id.liveChatView)
liveChatView.setSDK(sdk)
liveChatView.load()

// Glem ikke livscyklus-håndtering
override fun onResume() {
    super.onResume()
    sdk.refreshLiveEvents()
}

override fun onDestroy() {
    super.onDestroy()
    sdk.cleanup()
}
```