向您的应用添加实时聊天界面：

```kotlin
// 将 LiveChatView 添加到您的布局 XML
// <com.fastcomments.sdk.LiveChatView
//     android:id="@+id/liveChatView"
//     android:layout_width="match_parent"
//     android:layout_height="match_parent" />

// Create a configuration for the SDK
val config = CommentWidgetConfig().apply {
    tenantId = "your-tenant-id"
    urlId = "chat-room-identifier" 
    pageTitle = "Chat Room Name"
}
LiveChatView.setupLiveChatConfig(config)

// Optional: Add user authentication
val userData = SimpleSSOUserData(
    "User Name",
    "user@example.com",
    "https://path-to-avatar.jpg"
)
val sso = FastCommentsSSO(userData)
config.sso = sso.prepareToSend()

// Initialize the SDK
val sdk = FastCommentsSDK().configure(config)

// Set up the live chat view
val liveChatView = findViewById<LiveChatView>(R.id.liveChatView)
liveChatView.setSDK(sdk)
liveChatView.load()

// 别忘记生命周期处理
override fun onResume() {
    super.onResume()
    sdk.refreshLiveEvents()
}

override fun onDestroy() {
    super.onDestroy()
    sdk.cleanup()
}
```