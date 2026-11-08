सार्वजनिक `demo` टेनेंट के विरुद्ध स्थानीय रूप से हर विजेट और फ्लो देखने के लिए, रिपो क्लोन करें और चलाएँ:

```bash
yarn bootstrap
cd example
yarn ios       # या: yarn android, yarn web
```

शोकेस एंट्री पॉइंट `example/src/ShowcaseApp.tsx` है — एक ऐसा ऐप जो सभी विजेट, थीम और फ्लो उपलब्ध कराता है।

`yarn web` टारगेट `react-native-web` + `react-native-web-webview` (जो WebView को iframe के रूप में रेंडर करता है) का उपयोग करता है। ब्राउज़र में त्वरित विज़ुअल स्मोक टेस्ट के लिए उपयोगी; `injectJavaScript` और `onShouldStartLoadWithRequest` जैसे केवल-नेटिव WebView API वेब पर पूरी तरह काम नहीं करेंगे।