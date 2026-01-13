Προσθέστε μια διεπαφή συνομιλίας σε πραγματικό χρόνο στην εφαρμογή σας:

```kotlin
// Προσθέστε το LiveChatView στο XML του layout σας
// <com.fastcomments.sdk.LiveChatView
//     android:id="@+id/liveChatView"
//     android:layout_width="match_parent"
//     android:layout_height="match_parent" />

// Δημιουργήστε μια διαμόρφωση για το SDK
val config = CommentWidgetConfig().apply {
    tenantId = "your-tenant-id"
    urlId = "chat-room-identifier" 
    pageTitle = "Chat Room Name"
}
LiveChatView.setupLiveChatConfig(config)

// Προαιρετικό: Προσθέστε έλεγχο ταυτότητας χρήστη
val userData = SimpleSSOUserData(
    "User Name",
    "user@example.com",
    "https://path-to-avatar.jpg"
)
val sso = FastCommentsSSO(userData)
config.sso = sso.prepareToSend()

// Αρχικοποιήστε το SDK
val sdk = FastCommentsSDK().configure(config)

// Ρυθμίστε το LiveChatView
val liveChatView = findViewById<LiveChatView>(R.id.liveChatView)
liveChatView.setSDK(sdk)
liveChatView.load()

// Μην ξεχάσετε τη διαχείριση του κύκλου ζωής
override fun onResume() {
    super.onResume()
    sdk.refreshLiveEvents()
}

override fun onDestroy() {
    super.onDestroy()
    sdk.cleanup()
}
```