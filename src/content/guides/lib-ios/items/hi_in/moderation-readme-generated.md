### सभी उपयोगकर्ताओं के लिए उपलब्ध क्रियाएँ

- **Flag/Unflag** -- समीक्षा के लिए किसी टिप्पणी की रिपोर्ट करें

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- किसी उपयोगकर्ता की सभी टिप्पणियाँ छिपाएँ (प्रति-दर्शक)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### केवल एडमिन के लिए क्रियाएँ

- **Pin/Unpin** -- किसी टिप्पणी को थ्रेड के शीर्ष पर पिन करें

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- किसी टिप्पणी पर नए उत्तरों को रोकें, और अनलॉक होने तक संपादन और हटाने को ब्लॉक करें (यह सभी पर लागू होता है, मॉडरेटर्स सहित)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

सभी मॉडरेशन क्रियाएँ UI में टिप्पणी संदर्भ मेनू के माध्यम से भी उपलब्ध हैं। एडमिन क्रियाएँ केवल तब दिखाई देती हैं जब वर्तमान उपयोगकर्ता साइट एडमिन हो (SSO `isAdmin` फ्लैग या डैशबोर्ड कॉन्फ़िगरेशन के माध्यम से सेट)।