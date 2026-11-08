`LiveChatView` ऑटो-स्क्रॉल, दिनांक विभाजक और एक संक्षिप्त लेआउट के साथ रीयल-टाइम चैट अनुभव प्रदान करता है। यह SDK को सबसे पुराने-पहले सॉर्ट और तुरंत लाइव डिस्प्ले के लिए स्वचालित रूप से कॉन्फ़िगर करता है।

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // SSO अनुशंसित है ताकि उपयोगकर्ताओं के नाम हों
        )
        return FastCommentsSDK(config: config)
    }()

    var body: some View {
        LiveChatView(sdk: sdk)
            .onCommentPosted { comment in
                print("Sent: \(comment.commentHTML)")
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

`LiveChatView` इन कॉलबैक का समर्थन करता है:

- `.onCommentPosted` -- जब उपयोगकर्ता संदेश भेजता है तो सक्रिय होता है
- `.onCommentDeleted` -- जब कोई संदेश हटाया जाता है तो सक्रिय होता है
- `.onUserClick` -- जब किसी उपयोगकर्ता के नाम या अवतार पर टैप किया जाता है तो सक्रिय होता है

---
---