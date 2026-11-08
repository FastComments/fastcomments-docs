### डिपेंडेंसीज़ इंस्टॉल करें

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### सोर्स से बिल्ड करना

```bash
mkdir build
cd build
cmake ..
make
```

### इंस्टॉल करना

```bash
sudo make install
```

### लाइब्रेरी सामग्री

इस लाइब्रेरी में जेनरेट किया गया API क्लाइंट और SSO यूटिलिटीज़ शामिल हैं, जिनसे API के साथ काम करना आसान हो जाता है।

- [API क्लाइंट लाइब्रेरी दस्तावेज़](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### पब्लिक बनाम सिक्योर्ड API

API क्लाइंट में तीन क्लासेस हैं: `DefaultApi`, `PublicApi`, और `ModerationApi`। `DefaultApi` में वे मेथड्स हैं जिनके लिए आपकी API कुंजी आवश्यक है, और `PublicApi` में वे मेथड्स हैं जिन्हें बिना प्रमाणीकरण के सीधे ब्राउज़र/मोबाइल डिवाइस/आदि से कॉल किया जा सकता है। `ModerationApi` लाइव और तेज़ मॉडरेशन API का एक विस्तृत सूट प्रदान करता है। हर `ModerationApi` मेथड एक `sso` पैरामीटर स्वीकार करता है और SSO या FastComments.com सेशन कुकी के माध्यम से प्रमाणीकरण कर सकता है।