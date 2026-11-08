प्लगइन सेटिंग्स पेज **Site Administration > Plugins > Local plugins > FastComments** पर है। उपलब्ध विकल्प इस प्रकार हैं:

#### Tenant ID

आपकी FastComments Tenant ID। यह <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments डैशबोर्ड</a> में अपने खाता सेटिंग्स के अंतर्गत पाएं।

#### API Secret

आपकी API Secret कुंजी, जो Secure SSO मोड के लिए आवश्यक है। यह <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a> पर पाएं।

#### SSO Mode

चुनें कि उपयोगकर्ता कैसे प्रमाणित होंगे। प्रत्येक विकल्प के विवरण के लिए [SSO Modes](#moodle-sso-modes) अनुभाग देखें।

- **Secure** (अनुशंसित) - सर्वर-साइड HMAC-SHA256 से हस्ताक्षरित प्रमाणीकरण
- **Simple** - बिना हस्ताक्षर के क्लाइंट-साइड उपयोगकर्ता डेटा
- **None** - गुमनाम टिप्पणी, कोई Moodle लॉगिन एकीकरण नहीं

#### Page Contexts

नियंत्रित करता है कि टिप्पणियाँ कहाँ दिखाई दें:

- **Course pages** - कोर्स मुख्य पृष्ठों पर टिप्पणियाँ
- **Module/activity pages** - व्यक्तिगत गतिविधियों और संसाधनों पर टिप्पणियाँ
- **Both** - सभी पृष्ठ प्रकारों पर टिप्पणियाँ

#### Commenting Style

टिप्पणी अनुभव चुनें। प्रत्येक मोड के स्क्रीनशॉट के लिए [Commenting Styles](#moodle-commenting-styles) देखें।

- **Comments** - पृष्ठ सामग्री के नीचे मानक थ्रेडेड टिप्पणी विजेट
- **Collab Chat** - उपस्थिति संकेतकों के साथ इनलाइन टेक्स्ट चयन चर्चाएँ
- **Both** - टिप्पणियाँ और collab chat दोनों साथ-साथ सक्रिय

#### CDN URL

FastComments CDN URL। डिफ़ॉल्ट रूप से `https://cdn.fastcomments.com` होता है। यदि आपका डेटा EU क्षेत्र में होस्ट किया गया है तो इसे EU CDN URL में बदलें।