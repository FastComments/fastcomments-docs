सभी सेटिंग्स `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`) के अंतर्गत होती हैं।

## आवश्यक

- **Tenant ID** - आपकी FastComments Tenant ID। यह [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)) के अंतर्गत मिलती है।
- **API Secret** - Secure SSO, webhook सत्यापन, और page sync के लिए आवश्यक। यह [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)) के अंतर्गत मिलता है।

## API Secret को संग्रहीत करना

मॉड्यूल इन स्रोतों में से पहले से API Secret पढ़ता है जिसमें मान मौजूद हो:

1. **Key module** - [Key](https://www.drupal.org/project/key) इंस्टॉल करें, एक *Authentication* key बनाएं, और सेटिंग्स फ़ॉर्म पर **API Secret key** के अंतर्गत उसे चुनें। यह key एक environment variable या web root के बाहर की फ़ाइल द्वारा समर्थित हो सकती है।
2. **settings.php** - `settings.php` या `settings.local.php` में `$config['fastcomments.settings']['api_secret'] = 'your-secret';` जोड़ें। यह secret कभी भी exported configuration में नहीं जाता।
3. **सेटिंग्स फ़ॉर्म** - इसे **API Secret** फ़ील्ड में दर्ज करें। इससे मान `fastcomments.settings` में संग्रहीत होता है, इसलिए यह configuration exports में समाप्त हो जाता है। जब यह सक्रिय स्रोत होता है, तो Drupal status report चेतावनी देता है।

फ़ॉर्म दिखाता है कि कौन सा स्रोत सक्रिय है। फ़ील्ड खाली छोड़ने पर संग्रहीत secret बना रहता है। Key module या `settings.php` में secret स्थानांतरित करने के बाद इसे हटाने के लिए **Remove the API Secret stored in configuration** पर टिक करें।

## टिप्पणी शैली

वह विजेट चुनें जो इस बात से मेल खाता हो कि आप चाहते हैं कि लोग आपकी साइट पर कैसे बातचीत करें।

- **Live Comments** - रीयल-टाइम थ्रेडेड टिप्पणियाँ।
- **Streaming Chat** - लाइव चैट इंटरफ़ेस, इवेंट्स और लाइवस्ट्रीम के लिए उपयुक्त।
- **Collab Chat** - मुख्य कंटेंट क्षेत्र पर टेक्स्ट-चयन एनोटेशन। विज़िटर टेक्स्ट हाइलाइट करते हैं और संदर्भ में चर्चा शुरू करते हैं।
- **Collab Chat + Comments** - एक ही पेज पर collab chat और मानक टिप्पणियाँ दोनों।

## SSO Mode

- **None** - कोई SSO नहीं। उपयोगकर्ता अतिथि के रूप में टिप्पणी करते हैं या FastComments खाता बनाते हैं।
- **Simple** - Drupal उपयोगकर्ता जानकारी (नाम, ईमेल, अवतार) को बिना सर्वर-साइड सत्यापन के FastComments को भेजता है।
- **Secure** - Drupal उपयोगकर्ताओं को FastComments के साथ सत्यापित करने के लिए HMAC-SHA256 का उपयोग करता है। जब आपके पास API Secret कॉन्फ़िगर हो, तो यह अनुशंसित है।

विवरण के लिए `Single Sign-On (SSO)` खंड देखें।

## अन्य सेटिंग्स

- **CDN URL** - डिफ़ॉल्ट `https://cdn.fastcomments.com` है।
- **Site URL** - डिफ़ॉल्ट `https://fastcomments.com` है।
- **Email notifications** - जब किसी कंटेंट लेखक के कंटेंट पर नई टिप्पणी पोस्ट होती है, तो उसे ईमेल भेजें।

EU डेटा रेसिडेंसी के लिए, `EU Data Residency` खंड देखें।