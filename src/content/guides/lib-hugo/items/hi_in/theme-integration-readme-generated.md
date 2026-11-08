हर पोस्ट पर टिप्पणियाँ जोड़ने के लिए, जैसे Hugo का बिल्ट-इन Disqus सपोर्ट करता है, अपनी थीम के single टेम्पलेट (`layouts/_default/single.html`) में एक लाइन जोड़ें:

```go-html-template
\{{ partial "fastcomments/comments.html" . }}
```

यह partial केवल तब रेंडर होता है जब `tenantId` कॉन्फ़िगर किया गया हो। किसी अलग पेज पर टिप्पणियाँ front matter के साथ अक्षम करें:

```toml
+++
title = "A page with no comments"
comments = false
+++
```