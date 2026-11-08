### अवलोकन

FastComments Collab Chat मानक FastComments कमेंटिंग विजेट का विस्तार करता है, इसलिए यह बेस विजेट से सभी कॉन्फ़िगरेशन विकल्प प्राप्त करता है और साथ ही टेक्स्ट एनोटेशन के लिए कुछ विशिष्ट विकल्प भी जोड़ता है।

### आवश्यक कॉन्फ़िगरेशन

#### tenantId

आपकी FastComments Tenant ID आवश्यक है। आप इसे अपने [FastComments डैशबोर्ड](https://fastcomments.com/auth/my-account/api-secret) में पा सकते हैं।

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat विशिष्ट विकल्प

#### urlId

डिफ़ॉल्ट रूप से, Collab Chat पेज URL, एलिमेंट तक के DOM पाथ, और चयनित टेक्स्ट रेंज के आधार पर प्रत्येक बातचीत के लिए एक अद्वितीय पहचानकर्ता बनाता है। आप इसे कस्टम `urlId` से ओवरराइड कर सकते हैं।

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

यह तब उपयोगी है जब आपकी URL संरचना बदल सकती हो लेकिन आप वही बातचीत बनाए रखना चाहते हों, या जब आप कई पेजों पर एनोटेशन साझा करना चाहते हों।

#### topBarTarget

यह टॉप बार के प्रदर्शन को नियंत्रित करता है जो उपयोगकर्ता संख्या और चर्चा संख्या दिखाती है। टॉप बार को पूरी तरह अक्षम करने के लिए `null` सेट करें, या इसे किसी विशिष्ट स्थान पर रेंडर करने के लिए एक DOM एलिमेंट प्रदान करें।

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// टॉप बार अक्षम करें
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// टॉप बार को कस्टम स्थान पर रेंडर करें
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

जब आपके पेज का बैकग्राउंड गहरा हो तो डार्क मोड स्टाइलिंग सक्षम करें। यह पहचान स्वचालित होती है, लेकिन इसे ओवरराइड करना वांछनीय हो सकता है।

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

एक कॉलबैक फ़ंक्शन जो कमेंट संख्या बदलने पर चलता है। यह बैज या पेज टाइटल जैसे UI एलिमेंट अपडेट करने के लिए उपयोगी है।

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('कुल कमेंट:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### विरासत में मिले कॉन्फ़िगरेशन विकल्प

चूंकि Collab Chat मानक कमेंटिंग विजेट का विस्तार करता है, आप बेस FastComments विजेट के किसी भी कॉन्फ़िगरेशन विकल्प का उपयोग कर सकते हैं। यहाँ कुछ सामान्यतः उपयोग किए जाने वाले विकल्प हैं:

#### locale

विजेट UI के लिए भाषा सेट करें। FastComments दर्जनों भाषाओं का समर्थन करता है।

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // स्पेनिश
});
[inline-code-end]

#### readonly

सभी बातचीत को केवल-पढ़ने योग्य बनाएं। उपयोगकर्ता मौजूदा एनोटेशन देख सकते हैं लेकिन नए नहीं बना सकते या उत्तर नहीं दे सकते।

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso और simpleSSO

Single Sign-On का उपयोग करके अपने प्रमाणीकरण सिस्टम के साथ एकीकृत करें।

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO कॉन्फ़िगरेशन
    }
});
[inline-code-end]

प्रमाणीकरण विकल्पों के पूर्ण विवरण के लिए SSO दस्तावेज़ देखें।

#### maxReplyDepth

नियंत्रित करें कि उत्तर कितने स्तर तक गहरे जा सकते हैं। डिफ़ॉल्ट रूप से, Collab Chat इसे 0 पर सेट करता है, जिसका अर्थ है कि सभी कमेंट फ्लैट हैं (कोई नेस्टेड उत्तर नहीं)। यदि आप थ्रेडेड बातचीत चाहते हैं तो आप इसे बदल सकते हैं।

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3 स्तर की नेस्टिंग की अनुमति दें
});
[inline-code-end]

### आंतरिक कॉन्फ़िगरेशन

ये विकल्प Collab Chat द्वारा स्वचालित रूप से सेट किए जाते हैं और इन्हें ओवरराइड नहीं किया जाना चाहिए:

Collab Chat के लिए `productId` स्वचालित रूप से `3` पर सेट होता है। चैट विंडो कार्यक्षमता प्रदान करने के लिए `floating-chat` एक्सटेंशन स्वचालित रूप से लोड होता है। विजेट स्वचालित रूप से मोबाइल डिवाइस (768px से कम चौड़ी स्क्रीन) का पता लगाता है और UI को तदनुसार समायोजित करता है।

### पूर्ण उदाहरण

यहाँ कई कॉन्फ़िगरेशन विकल्पों को एक साथ दिखाने वाला एक उदाहरण है:

[inline-code-attrs-start title = "कॉन्फ़िगरेशन उदाहरण"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // आपका SSO कॉन्फ़िगरेशन
    },
    maxReplyDepth: 1
});
[inline-code-end]

बेस विजेट से विरासत में मिले सभी उपलब्ध कॉन्फ़िगरेशन विकल्पों की पूरी सूची के लिए, मुख्य FastComments कॉन्फ़िगरेशन दस्तावेज़ देखें।