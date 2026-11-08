Webhooks को REST API के माध्यम से भी प्रबंधित किया जा सकता है। इसी तरह Zapier जैसी इंटीग्रेशन डैशबोर्ड को छुए बिना कमेंट इवेंट्स की सदस्यता लेती हैं, और यह REST Hooks पैटर्न का पालन करता है: सब्सक्राइब करें, इवेंट्स प्राप्त करें, अनसब्सक्राइब करें।

API सब्सक्रिप्शन डैशबोर्ड में कॉन्फ़िगर किए गए webhooks के साथ-साथ रहते हैं। एक कमेंट इवेंट हर उस webhook को डिलीवर किया जाता है जो उसके डोमेन से मेल खाता है, प्रत्येक अपनी अलग डिलीवरी के रूप में, चाहे webhook किसी भी तरह से बनाया गया हो।

## प्रमाणीकरण

हर अनुरोध में `x-api-key` हेडर (या `API_KEY` क्वेरी पैरामीटर) में आपकी API Key और `tenantId` क्वेरी पैरामीटर में आपकी tenant ID चाहिए। दोनों डैशबोर्ड के API Secret पेज पर दिखाए जाते हैं।

## सब्सक्राइब करें

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| फ़ील्ड | आवश्यक | विवरण |
|-------|----------|-------------|
| `url` | हाँ | एक पूर्ण http या https URL। |
| `event` | हाँ | `comment-created`, `comment-updated` या `comment-deleted`। |
| `domain` | नहीं | आपके खाते के कॉन्फ़िगरेशन से एक डोमेन। डिफ़ॉल्ट `*` है, जो हर डोमेन के इवेंट्स प्राप्त करता है। |
| `method` | नहीं | `POST` (डिफ़ॉल्ट), `PUT` या `DELETE`। |

प्रतिक्रिया में सब्सक्रिप्शन होता है:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

उसी URL को उसी इवेंट और डोमेन के लिए दोबारा सब्सक्राइब करने पर डुप्लिकेट बनाने के बजाय मौजूदा सब्सक्रिप्शन लौटाया जाता है, इसलिए क्लाइंट सुरक्षित रूप से रीट्राई कर सकता है। प्रत्येक tenant के पास अधिकतम 50 API सब्सक्रिप्शन हो सकते हैं।

## सूची

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Tenant के हर webhook को लौटाता है, जिनमें डैशबोर्ड में प्रबंधित वाले भी शामिल हैं (`"source": "dashboard"`)। `event`, `domain` या `source` से फ़िल्टर करें।

## अनसब्सक्राइब करें

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

किसी सब्सक्रिप्शन को डिलीट करने से उसके लिए कतार में मौजूद इवेंट्स भी हटा दिए जाते हैं। केवल API के माध्यम से बनाई गई सब्सक्रिप्शन ही इस तरह डिलीट की जा सकती हैं; डैशबोर्ड का webhook, या आपके खाते पर मौजूद न होने वाली कोई id, `not-found` कोड के साथ `404` लौटाता है। डैशबोर्ड webhooks को Webhooks पेज पर संपादित किया जाता है।

## पेलोड और साइनिंग

डिलीवरी डैशबोर्ड webhooks के समान पेलोड का उपयोग करती हैं (Data Structures देखें) और उसी HMAC स्कीम से साइन की जाती हैं (Security & API Tokens देखें)। API सब्सक्रिप्शन को कभी भी लेगेसी `token` हेडर प्राप्त नहीं होता, इसलिए इसके बजाय `X-FastComments-Signature` हेडर सत्यापित करें।

## नमूना पेलोड

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

खाते की सबसे हाल की कमेंट्स को ठीक उसी आकार में लौटाता है जिस आकार में डिलीवरी होती है, ताकि कोई इंटीग्रेशन पहले इवेंट के आने से पहले वास्तविक नमूना डेटा दिखा सके। `event` वैकल्पिक है और केवल सत्यापित किया जाता है, क्योंकि हर इवेंट एक ही कमेंट ऑब्जेक्ट डिलीवर करता है। `limit` का डिफ़ॉल्ट 3 है और 1 से 10 स्वीकार करता है। 2 API क्रेडिट खर्च होते हैं।

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## 410 Gone के साथ प्रतिक्रिया देना

यदि किसी API सब्सक्रिप्शन का एंडपॉइंट HTTP `410 Gone` के साथ प्रतिक्रिया देता है, तो FastComments इसे अनसब्सक्राइब के रूप में मानता है: सब्सक्रिप्शन उसके कतार में मौजूद इवेंट्स के साथ डिलीट कर दिया जाता है, और आगे कोई डिलीवरी का प्रयास नहीं किया जाता। डैशबोर्ड में कॉन्फ़िगर किए गए webhooks कभी भी स्वचालित रूप से डिलीट नहीं किए जाते; उनके लिए 410 एक सामान्य विफलता है। कोई अन्य विफलता स्थिति रीट्राई की जाती है और अंततः webhook अक्षम कर दिया जाता है, जैसा कि How it Works & Handling Retries में वर्णित है।

## डैशबोर्ड

API सब्सक्रिप्शन Webhooks सूची में स्रोत **API** के साथ दिखाई देते हैं, जहाँ एक व्यवस्थापक उन्हें संपादित, अक्षम, पुनः सक्षम या डिलीट कर सकता है।