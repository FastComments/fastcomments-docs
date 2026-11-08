गिनतियाँ प्राप्त करने के लिए कुछ endpoints हैं, जो इस बात पर निर्भर करता है कि आप क्या चाहते हैं और आप उन्हें ब्राउज़र से, सर्वर से, या API SDK का उपयोग करके प्राप्त करना चाहते हैं।

## सार्वजनिक कमेंट गिनतियाँ

आप ऊपर दिए गए widgets का उपयोग करके या उन APIs का उपयोग करके सार्वजनिक कमेंट गिनतियाँ प्राप्त कर सकते हैं जिनका वे उपयोग करते हैं। ये APIs 2019 से अपरिवर्तित हैं और कभी नहीं बदलेंगी।

[inline-code-attrs-start title = 'सिंगल काउंट एंडपॉइंट'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

यह इस तरह की एक संरचना लौटाएगा:

[inline-code-attrs-start title = 'सिंगल काउंट एंडपॉइंट प्रतिक्रिया'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

`postfix` प्रॉपर्टी हमेशा शामिल रहती है।

[inline-code-attrs-start title = 'बल्क काउंट एंडपॉइंट'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

यह इस तरह की एक संरचना लौटाएगा:

[inline-code-attrs-start title = 'बल्क काउंट एंडपॉइंट प्रतिक्रिया'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

`counts` ऑब्जेक्ट केवल उन पेजों के लिए भरा जाता है जिनकी गिनतियाँ होती हैं। `translations` मैप हमेशा मौजूद रहता है क्योंकि इसका उपयोग widget के लिए किया जाता है।

### सार्वजनिक एंडपॉइंट व्यवहार / कैशिंग

सार्वजनिक endpoints में ट्रैफ़िक के शिखरों को संभालने के लिए 60-सेकंड की कैशिंग प्रणाली है। आंतरिक रूप से यह सर्वर पर मेमोरी में प्रति-थ्रेड LRU कैश है, इसलिए जब लोग बहुत सारे कमेंट छोड़ रहे हों तो आप गिनतियों में थोड़ा बदलाव देख सकते हैं (अस्थायी रूप से ऊपर जाकर फिर नीचे आ जाना)।

सार्वजनिक endpoints हमेशा *कुल* कमेंट गिनती लौटाते हैं, न कि रूट कमेंट गिनती।

### सर्वर-साइड APIs / SDK

अपने सर्वर से कमेंट प्राप्त करने का तरीका है [Pages API](/guide-api.html#page-structure) को कॉल करना और पेज ऑब्जेक्ट प्राप्त करना, जिसमें कुल कमेंट गिनती और रूट कमेंट गिनती शामिल होती है। हम SDKs प्रदान करते हैं जो आपको API अनुरोध को मैन्युअल रूप से बनाए बिना इस API को कॉल करने की अनुमति देते हैं, और टाइप किए गए रिटर्न मान प्रदान करते हैं।