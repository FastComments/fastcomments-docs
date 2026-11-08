### Nimble का उपयोग करना

```bash
nimble install fastcomments
```

### स्रोत से बिल्ड करना

```bash
nimble build
```

### लाइब्रेरी की सामग्री

इस लाइब्रेरी में जेनरेट किया गया API क्लाइंट और SSO यूटिलिटीज़ शामिल हैं, जो API के साथ काम करना आसान बनाते हैं।

- [API क्लाइंट लाइब्रेरी दस्तावेज़ीकरण](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### पब्लिक बनाम सुरक्षित APIs

API क्लाइंट में तीन API मॉड्यूल हैं: `api_default`, `api_public`, और `api_moderation`। `api_default` में वे मेथड्स हैं जिनके लिए आपकी API key आवश्यक है, और `api_public` में वे API कॉल शामिल हैं
जिन्हें बिना प्रमाणीकरण के सीधे ब्राउज़र/मोबाइल डिवाइस/आदि से किया जा सकता है। `api_moderation` मॉड्यूल में मॉडरेटर डैशबोर्ड के लिए मेथड्स हैं।

`api_moderation` मॉड्यूल लाइव और तेज़ मॉडरेशन APIs का एक व्यापक सूट प्रदान करता है। हर `api_moderation` मेथड एक `sso` पैरामीटर स्वीकार करता है और SSO या FastComments.com सेशन कुकी के माध्यम से प्रमाणित हो सकता है।