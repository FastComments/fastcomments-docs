### FastComments API

FastComments कई संसाधनों (resources) के साथ इंटरैक्ट करने के लिए एक API प्रदान करता है। हमारे प्लेटफ़ॉर्म के साथ इंटीग्रेशन बनाएं, या अपने स्वयं के क्लाइंट भी बनाएं!

इस दस्तावेज़ीकरण में, आपको API द्वारा समर्थित सभी संसाधनों का दस्तावेज़ीकरण उनके रिक्वेस्ट और रिस्पॉन्स प्रकारों के साथ मिलेगा।

Enterprise ग्राहकों के लिए, सभी API एक्सेस Audit Log में कैप्चर किया जाता है।

### जेनरेट किए गए SDKs

FastComments अब हमारे कोड से एक [API Spec](https://fastcomments.com/js/swagger.json) जेनरेट करता है (यह अभी पूरा नहीं है, लेकिन इसमें कई APIs शामिल हैं)।

हमारे पास अब लोकप्रिय भाषाओं के लिए SDKs भी हैं:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### प्रमाणीकरण (Authentication)

API आपकी [api key](https://fastcomments.com/auth/my-account/api-secret) को `X-API-KEY` हेडर या `API_KEY` क्वेरी पैरामीटर के रूप में पास करके प्रमाणित की जाती है। API कॉल करने के लिए आपको अपनी `tenantId` भी चाहिए होगी। यह आपकी api key वाले उसी पेज से प्राप्त की जा सकती है।

### सुरक्षा नोट

इन रूट्स को **सर्वर** से कॉल करने के लिए ही है। इन्हें ब्राउज़र से कॉल __न करें__। ऐसा करने से आपकी API key उजागर हो जाएगी - इससे आपके खाते तक पूर्ण एक्सेस उन सभी को मिल जाएगा जो किसी पेज का सोर्स कोड देख सकते हैं!

#### प्रमाणीकरण विकल्प एक - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### प्रमाणीकरण विकल्प दो - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### प्रमाणीकरण विकल्प तीन - OAuth Bearer Token

- Header: `Authorization: Bearer fcat_...`

Zapier जैसे थर्ड-पार्टी एप्लिकेशन और [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) के क्लाइंट API key के बजाय OAuth के माध्यम से एक टोकन प्राप्त करते हैं। वह टोकन यहाँ हर एंडपॉइंट पर काम करता है। टोकन से tenant निहित रहता है, इसलिए `tenantId` वैकल्पिक है, लेकिन दिया जाने पर यह टोकन से मेल खाना चाहिए। `GET` रिक्वेस्ट को `read` स्कोप की आवश्यकता होती है और हर अन्य मेथड को `write` स्कोप की। पूरा फ़्लो, जिसमें क्लाइंट रजिस्ट्रेशन, PKCE, रिफ्रेश और रिवोकेशन शामिल है, [OAuth Authorization](#oauth) के अंतर्गत दस्तावेज़ित है। डिस्कवरी `https://fastcomments.com/.well-known/oauth-authorization-server` से शुरू होती है।

### अपने स्वयं के राइट्स पढ़ना (Reading Your Own Writes)

FastComments Active-Active उपलब्धता प्रदान करता है। आपके डेटासेंटर से रिक्वेस्ट [आपके निकटतम पॉइंट ऑफ़ प्रेज़ेंस](https://sophon.fastcomments.com/) पर रूट की जाती हैं। यह स्वचालित है, और सामान्यतः आप read-your-write सिमेंटिक्स देख सकते हैं। यदि आप यह सुनिश्चित करना चाहते हैं कि आप अपने स्वयं के राइट्स पढ़ें, तो आप उस क्षेत्र को अपने API होस्ट के रूप में उपयोग करके अपनी रिक्वेस्ट को एक निश्चित क्षेत्र (region) पर पिन कर सकते हैं (हालाँकि अधिकांश इंटीग्रेशन के लिए यह सामान्यतः आवश्यक नहीं है):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

ध्यान दें कि यदि आप ऐसा करते हैं तो आप एक फ़ॉलबैक परिभाषित करना चाह सकते हैं, क्योंकि हमने अतीत में एंट्रीपॉइंट नोड्स को deprecated किया है और स्विचओवर के लिए नए नामों का उपयोग किया है।