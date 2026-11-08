SDK मेथड्स `FastCommentsError` थ्रो करते हैं, जो `LocalizedError` के अनुरूप है:

```swift
do {
    try await sdk.load()
} catch let error as FastCommentsError {
    print(error.translatedError ?? error.reason ?? "Unknown error")
} catch {
    print(error.localizedDescription)
}
```

`FastCommentsError` की प्रॉपर्टीज़:

- `code` -- API से मिला एरर कोड
- `reason` -- अंग्रेज़ी में एरर विवरण
- `translatedError` -- सर्वर द्वारा प्रदान किया गया स्थानीयकृत एरर संदेश

ब्लॉकिंग एरर्स `sdk.blockingErrorMessage` के माध्यम से भी स्वतः दिखाए जाते हैं, जिन्हें बिल्ट-इन व्यूज़ उपयोगकर्ता को प्रदर्शित करते हैं।