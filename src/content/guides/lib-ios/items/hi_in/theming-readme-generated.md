### थीम प्रीसेट्स

चार बिल्ट-इन प्रीसेट्स उपलब्ध हैं:

```swift
// सिस्टम डिफ़ॉल्ट
sdk.theme = FastCommentsTheme.default

// शैडो और बड़े गोल कोनों वाले कार्ड
sdk.theme = FastCommentsTheme.modern

// फ्लैट, बिना शैडो, छोटा कॉर्नर रेडियस, बिना थ्रेड लाइन
sdk.theme = FastCommentsTheme.minimal

// सभी एक्शन रंगों को एक सिंगल ब्रांड रंग पर सेट करें
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### कमेंट डिस्प्ले स्टाइल

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // डिवाइडर के साथ फ्लैट लिस्ट (डिफ़ॉल्ट)
theme.commentStyle = .card    // शैडो के साथ गोल कार्ड
theme.commentStyle = .bubble  // चैट बबल स्टाइल
```

### रंग

सभी रंग प्रॉपर्टीज़ वैकल्पिक हैं। सेट न किए गए मान उचित सिस्टम डिफ़ॉल्ट पर वापस चले जाते हैं।

```swift
var theme = FastCommentsTheme()

// ब्रांड रंग
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// बैकग्राउंड
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// एक्शन बटन
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// वोट
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// लिंक
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// डायलॉग
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// इनपुट बार
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// अन्य
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### टाइपोग्राफी

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### लेआउट और स्पेसिंग

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // कमेंट पंक्तियों के बीच पॉइंट
theme.nestingIndent = 20          // प्रत्येक नेस्टिंग स्तर पर इंडेंटेशन के पॉइंट
theme.avatarSize = 36             // रूट कमेंट के लिए अवतार व्यास
theme.replyAvatarSize = 28        // नेस्टेड रिप्लाई के लिए अवतार व्यास
```

### विज़ुअल इफ़ेक्ट्स

```swift
theme.showShadows = true          // कार्ड पर हल्की शैडो
theme.showThreadLine = true       // नेस्टेड रिप्लाई को जोड़ने वाली वर्टिकल लाइन
theme.animateVotes = true         // वोट परिवर्तन पर स्प्रिंग एनिमेशन
```

### थीम लागू करना

दो तरीके:

```swift
// SwiftUI environment के माध्यम से (व्यू हायरार्की के लिए अनुशंसित)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// सीधे SDK पर
sdk.theme = theme
```