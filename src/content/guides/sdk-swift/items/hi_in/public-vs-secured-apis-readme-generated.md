FastComments SDK दो प्रकार के API endpoints प्रदान करता है:

### PublicAPI - Client-Safe Endpoints

`PublicAPI` में वे endpoints शामिल हैं जिन्हें client-side कोड (iOS/macOS ऐप्स) से कॉल करना सुरक्षित है। ये endpoints:
- API key की आवश्यकता नहीं रखते
- प्रमाणीकरण के लिए SSO tokens का उपयोग कर सकते हैं
- प्रति user/device rate-limited होते हैं
- end-user के सामने वाले एप्लिकेशन के लिए उपयुक्त हैं

**उदाहरण उपयोग**: अपने iOS ऐप में comments प्राप्त करना और बनाना

### DefaultAPI - Server-Side Endpoints

`DefaultAPI` में प्रमाणित endpoints शामिल हैं जिन्हें API key की आवश्यकता होती है। ये endpoints:
- आपकी FastComments API key की आवश्यकता रखते हैं
- केवल server-side कोड से ही कॉल किए जाने चाहिए
- आपके FastComments डेटा तक पूर्ण पहुँच प्रदान करते हैं
- प्रति tenant rate-limited होते हैं

**उदाहरण उपयोग**: प्रशासनिक कार्य, बल्क डेटा एक्सपोर्ट, मॉडरेशन टूल्स

**महत्वपूर्ण**: अपनी API key को कभी भी client-side कोड में expose न करें। API keys का उपयोग केवल server-side ही किया जाना चाहिए।