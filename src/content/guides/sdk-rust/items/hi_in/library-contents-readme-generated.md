FastComments Rust SDK कई मॉड्यूल्स से मिलकर बना है:

- **Client Module** - FastComments REST APIs के लिए API क्लाइंट
  - सभी API मॉडल्स के लिए पूर्ण टाइप परिभाषाएँ
  - सभी FastComments मेथड्स को कवर करने वाले तीन API क्लाइंट:
    - `default_api` (**DefaultApi**) - सर्वर-साइड उपयोग के लिए API-की-प्रमाणित मेथड्स
    - `public_api` (**PublicApi**) - सार्वजनिक, बिना API-की वाले मेथड्स जिन्हें ब्राउज़रों और मोबाइल ऐप्स से कॉल करना सुरक्षित है
    - `moderation_api` (**ModerationApi**) - लाइव और तेज़ मॉडरेशन APIs का एक विस्तृत सूट। प्रत्येक Moderation मेथड एक `sso` पैरामीटर स्वीकार करता है और SSO या FastComments.com सेशन कुकी के माध्यम से प्रमाणित हो सकता है।
  - tokio के साथ पूर्ण async/await समर्थन
  - विस्तृत API दस्तावेज़ीकरण के लिए [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) देखें

- **SSO Module** - सर्वर-साइड सिंगल साइन-ऑन यूटिलिटीज़
  - उपयोगकर्ता प्रमाणीकरण के लिए सुरक्षित टोकन जनरेशन
  - सरल और सुरक्षित दोनों SSO मोड्स के लिए समर्थन
  - HMAC-SHA256 आधारित टोकन साइनिंग

- **Core Types** - साझा टाइप परिभाषाएँ और यूटिलिटीज़
  - कमेंट मॉडल्स और मेटाडेटा संरचनाएँ
  - उपयोगकर्ता और टेनेंट कॉन्फ़िगरेशन
  - सामान्य ऑपरेशनों के लिए हेल्पर फ़ंक्शन