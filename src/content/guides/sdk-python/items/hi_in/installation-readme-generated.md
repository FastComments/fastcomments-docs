### GitHub से इंस्टॉल करें

सीधे एक रिलीज़ टैग से इंस्टॉल करें (अनुशंसित, पूरी तरह पुनरुत्पादन योग्य):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

ब्रांच के बजाय टैग को पिन करें ताकि बिल्ड नियत (deterministic) रहें। वही रूप `requirements.txt` में भी काम करता है:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

प्रत्येक टैग किए गए [GitHub Release](https://github.com/FastComments/fastcomments-python/releases) में एक बनी हुई wheel भी संलग्न होती है, यदि आप बाइनरी आर्टिफैक्ट सीधे इंस्टॉल करना पसंद करते हैं।

### लाइब्रेरी की सामग्री

इस लाइब्रेरी में दो मॉड्यूल हैं: जनरेट किया गया API क्लाइंट और कोर Python लाइब्रेरी, जिसमें हाथ से लिखे गए यूटिलिटीज़ हैं जो API के साथ काम करना आसान बनाते हैं, जिसमें SSO सपोर्ट शामिल है।

- [API क्लाइंट लाइब्रेरी दस्तावेज़](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [कोर लाइब्रेरी दस्तावेज़, SSO उदाहरणों सहित](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### पब्लिक बनाम सिक्योर्ड APIs

API क्लाइंट में तीन क्लासेस हैं: `DefaultApi`, `PublicApi`, और `ModerationApi`। `DefaultApi` में वे मेथड हैं जिनके लिए आपकी API key आवश्यक है, और `PublicApi` में वे मेथड हैं जिन्हें बिना प्रमाणीकरण के सीधे ब्राउज़र/मोबाइल डिवाइस/आदि से कॉल किया जा सकता है। `ModerationApi` लाइव और तेज़ मॉडरेशन APIs का एक व्यापक सूट प्रदान करता है। हर `ModerationApi` मेथड एक `sso` पैरामीटर स्वीकार करता है और SSO या FastComments.com सेशन कुकी के माध्यम से प्रमाणित हो सकता है।