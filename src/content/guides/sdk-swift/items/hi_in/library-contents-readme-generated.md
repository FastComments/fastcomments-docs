FastComments Swift SDK कई मॉड्यूल्स से मिलकर बना है:

- **Client Module** - FastComments REST APIs के लिए API क्लाइंट
  - सभी API मॉडल्स के लिए संपूर्ण टाइप परिभाषाएँ
  - प्रमाणित (`DefaultAPI`), सार्वजनिक (`PublicAPI`), और मॉडरेशन (`ModerationAPI`) मेथड्स
  - पूर्ण async/await समर्थन
  - विस्तृत API दस्तावेज़ीकरण के लिए [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) देखें

- **SSO Module** - सर्वर-साइड Single Sign-On यूटिलिटीज़
  - उपयोगकर्ता प्रमाणीकरण के लिए सुरक्षित टोकन जनरेशन
  - सरल और सुरक्षित दोनों SSO मोड्स के लिए समर्थन
  - CryptoKit का उपयोग करके HMAC-SHA256 आधारित टोकन साइनिंग