#### Canvas में Developer Keys खोलें

Canvas में व्यवस्थापक (administrator) के रूप में लॉग इन करें। **Admin** (बाएँ साइडबार में) पर जाएँ > अपना खाता चुनें > **Developer Keys**।

#### एक LTI Developer Key बनाएँ

**+ Developer Key** पर क्लिक करें और **LTI Key** चुनें।

कॉन्फ़िगरेशन फ़ॉर्म में:

1. **Redirect URIs** फ़ील्ड (बाईं ओर) में, FastComments सेटअप पेज से **Launch URL** पेस्ट करें।
2. दाईं ओर, **Method** को **Enter URL** पर सेट करें।
3. FastComments से कॉपी किया गया **Configuration URL** **JSON URL** फ़ील्ड में पेस्ट करें।
4. Canvas LTI कॉन्फ़िगरेशन को स्वतः लोड कर लेगा।
5. कुंजी को एक नाम दें (जैसे "FastComments")।
6. **Save** पर क्लिक करें।

#### Developer Key सक्रिय करें

सहेजने के बाद, नई कुंजी Developer Keys तालिका में दिखाई देगी जिसकी **State** **OFF** पर होगी। इसे **ON** पर सेट करने के लिए टॉगल पर क्लिक करें। Canvas आपसे पुष्टि करने के लिए कह सकता है — कुंजी सक्रिय करने के लिए **Allow** पर क्लिक करें।

#### Client ID कॉपी करें

Developer Keys तालिका Details कॉलम में एक संख्यात्मक **Client ID** दिखाती है (जैसे `17000000000042`)। यह संख्या कॉपी करें - अगले चरण में आपको इसकी आवश्यकता होगी।