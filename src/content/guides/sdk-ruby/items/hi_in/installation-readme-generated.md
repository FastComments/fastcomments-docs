अपने एप्लिकेशन की Gemfile में यह लाइन जोड़ें:

```ruby
gem 'fastcomments'
```

और फिर निष्पादित करें:

```bash
bundle install
```

या इसे स्वयं इस प्रकार इंस्टॉल करें:

```bash
gem install fastcomments
```

### लाइब्रेरी की सामग्री

इस लाइब्रेरी में जनरेट किया गया API क्लाइंट और SSO यूटिलिटीज़ शामिल हैं, जो API के साथ काम करना आसान बनाती हैं।

- [API क्लाइंट लाइब्रेरी दस्तावेज़](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### पब्लिक बनाम सुरक्षित APIs

API क्लाइंट में तीन क्लास हैं: `DefaultApi`, `PublicApi`, और `ModerationApi`। `DefaultApi` में वे मेथड्स हैं जिनमें आपकी API key की आवश्यकता होती है, और `PublicApi` में वे API कॉल्स हैं जो बिना प्रमाणीकरण के सीधे ब्राउज़र/मोबाइल डिवाइस/आदि से की जा सकती हैं। `ModerationApi` में वे मेथड्स हैं जो मॉडरेटर डैशबोर्ड को संचालित करते हैं।

`ModerationApi` लाइव और तेज़ मॉडरेशन APIs का एक व्यापक सूट प्रदान करता है। हर `ModerationApi` मेथड एक `sso` पैरामीटर स्वीकार करता है और SSO या FastComments.com सेशन कुकी के माध्यम से प्रमाणित हो सकता है।