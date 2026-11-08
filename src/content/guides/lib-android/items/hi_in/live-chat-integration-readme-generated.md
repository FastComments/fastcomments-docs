अपने ऐप में एक रीयल-टाइम चैट इंटरफ़ेस जोड़ें:

```kotlin
// अपने लेआउट XML में LiveChatView जोड़ें
// <com.fastcomments.sdk.LiveChatView
//     android:id="@+id/liveChatView"
//     android:layout_width="match_parent"
//     android:layout_height="match_parent" />

// SDK के लिए एक कॉन्फ़िगरेशन बनाएं
val config = CommentWidgetConfig().apply {
    tenantId = "your-tenant-id"
    urlId = "chat-room-identifier" 
    pageTitle = "Chat Room Name"
}
LiveChatView.setupLiveChatConfig(config)

// वैकल्पिक: यूज़र ऑथेंटिकेशन जोड़ें
val userData = SimpleSSOUserData(
    "User Name",
    "user@example.com",
    "https://path-to-avatar.jpg"
)
val sso = FastCommentsSSO(userData)
config.sso = sso.prepareToSend()

// SDK को इनिशियलाइज़ करें
val sdk = FastCommentsSDK().configure(config)

// लाइव चैट व्यू सेट अप करें
val liveChatView = findViewById<LiveChatView>(R.id.liveChatView)
liveChatView.setSDK(sdk)
liveChatView.load()

// लाइफ़साइकिल हैंडलिंग करना न भूलें
override fun onResume() {
    super.onResume()
    sdk.refreshLiveEvents()
}

override fun onDestroy() {
    super.onDestroy()
    sdk.cleanup()
}
```