### पेज साइज़

```swift
// Comments: default 30
sdk.pageSize = 50

// Feed: default 10
feedSDK.pageSize = 20
```

### अधिक कमेंट्स लोड करना

UI पेजिनेशन कंट्रोल्स अपने आप दिखाता है। आप प्रोग्रामेटिक रूप से भी पेजिनेशन ट्रिगर कर सकते हैं:

```swift
// Load next page
try await sdk.loadMore()

// Load all remaining (disabled if >2000 comments for performance)
try await sdk.loadAll()

// Check state
sdk.hasMore            // Whether more pages exist
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### चाइल्ड कमेंट पेजिनेशन

नेस्टेड रिप्लाई लेज़ी लोड होते हैं। जब कोई यूज़र एक थ्रेड खोलता है, तो पहले 5 चाइल्ड कमेंट्स लोड होते हैं। यदि और रिप्लाई मौजूद हैं तो एक "load more replies" कंट्रोल दिखाई देता है। यह UI द्वारा अपने आप हैंडल किया जाता है।

---