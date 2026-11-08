### अवलोकन

FastComments Image Chat मानक FastComments कमेंटिंग विजेट का विस्तार करता है, इसलिए यह बेस विजेट से सभी कॉन्फ़िगरेशन विकल्प प्राप्त करता है और साथ में इमेज एनोटेशन के लिए कुछ विशिष्ट विकल्प भी जोड़ता है।

### आवश्यक कॉन्फ़िगरेशन

#### tenantId

आपकी FastComments Tenant ID आवश्यक है। आप इसे अपने [FastComments डैशबोर्ड](https://fastcomments.com/auth/my-account/api-secret) में पा सकते हैं।

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat विशिष्ट विकल्प

#### urlId

डिफ़ॉल्ट रूप से, Image Chat पेज URL, इमेज स्रोत और X/Y निर्देशांकों के आधार पर प्रत्येक बातचीत के लिए एक अद्वितीय पहचानकर्ता बनाता है। आप इसे कस्टम `urlId` से बदल सकते हैं।

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

यह तब उपयोगी होता है जब आपकी URL संरचना बदल सकती हो लेकिन आप वही बातचीत बनाए रखना चाहते हों, या जब आप कई पेजों पर एनोटेशन साझा करना चाहते हों।

#### chatSquarePercentage

क्लिक करने योग्य चैट मार्करों के आकार को इमेज की चौड़ाई के प्रतिशत के रूप में नियंत्रित करता है। डिफ़ॉल्ट 5% है, यानी प्रत्येक मार्कर इमेज की चौड़ाई का 5% होता है।

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // बड़े, अधिक दिखाई देने वाले मार्कर
});
```

छोटे मान कम बाधक मार्कर बनाते हैं जो विस्तृत इमेज के लिए बेहतर काम करते हैं। बड़े मान मार्करों को व्यस्त इमेज पर या मोबाइल डिवाइस उपयोगकर्ताओं के लिए देखने और क्लिक करने में आसान बनाते हैं।

#### hasDarkBackground

जब आपके पेज का बैकग्राउंड गहरा हो तो डार्क मोड स्टाइलिंग सक्षम करें।

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

एक कॉलबैक फ़ंक्शन जो कमेंट की संख्या बदलने पर चलता है। यह बैज या पेज टाइटल जैसे UI तत्वों को अपडेट करने के लिए उपयोगी है।

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### विरासत में मिले कॉन्फ़िगरेशन विकल्प

चूंकि Image Chat मानक कमेंटिंग विजेट का विस्तार करता है, आप बेस FastComments विजेट का कोई भी कॉन्फ़िगरेशन विकल्प उपयोग कर सकते हैं। यहाँ कुछ सामान्यतः उपयोग किए जाने वाले विकल्प हैं:

#### locale

विजेट UI के लिए भाषा सेट करें। FastComments दर्जनों भाषाओं का समर्थन करता है।

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // स्पेनिश
});
```

#### readonly

सभी बातचीत को केवल-पढ़ने योग्य बनाएं। उपयोगकर्ता मौजूदा मार्कर और चर्चाएँ देख सकते हैं लेकिन नई नहीं बना सकते या जवाब नहीं दे सकते।

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso और simpleSSO

Single Sign-On का उपयोग करके अपने प्रमाणीकरण सिस्टम के साथ एकीकृत करें।

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO कॉन्फ़िगरेशन
    }
});
```

प्रमाणीकरण विकल्पों के पूर्ण विवरण के लिए SSO दस्तावेज़ देखें।

#### maxReplyDepth

नियंत्रित करें कि जवाब कितने स्तर तक गहरे जा सकते हैं। डिफ़ॉल्ट रूप से, Image Chat इसे 0 पर सेट करता है, यानी सभी कमेंट फ्लैट होते हैं (कोई नेस्टेड जवाब नहीं)। यदि आप थ्रेडेड बातचीत चाहते हैं तो आप इसे बदल सकते हैं।

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 3 स्तर की नेस्टिंग की अनुमति दें
});
```

### आंतरिक कॉन्फ़िगरेशन

ये विकल्प Image Chat द्वारा स्वचालित रूप से सेट किए जाते हैं और इन्हें ओवरराइड नहीं किया जाना चाहिए:

Image Chat के लिए `productId` स्वचालित रूप से `2` पर सेट होता है। चैट विंडो कार्यक्षमता प्रदान करने के लिए `floating-chat` एक्सटेंशन स्वचालित रूप से लोड होता है। विजेट स्वचालित रूप से मोबाइल डिवाइसों (768px से कम चौड़ाई वाली स्क्रीन) का पता लगाता है और फुलस्क्रीन चैट विंडो के साथ UI को तदनुसार समायोजित करता है।

### टारगेट एलिमेंट की लचीलापन

`FastCommentsImageChat` का पहला पैरामीटर सीधे एक `<img>` एलिमेंट हो सकता है या अंदर इमेज वाला एक कंटेनर एलिमेंट:

```javascript
// सीधा इमेज एलिमेंट
FastCommentsImageChat(document.getElementById('my-image'), config);

// अंदर इमेज वाला कंटेनर
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

यदि आप कंटेनर एलिमेंट पास करते हैं तो विजेट इमेज को स्वचालित रूप से ढूंढ लेगा।

### पूर्ण उदाहरण

यहाँ कई कॉन्फ़िगरेशन विकल्पों को एक साथ दिखाने वाला एक उदाहरण है:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // आपका SSO कॉन्फ़िगरेशन
    },
    maxReplyDepth: 1
});
```

बेस विजेट से प्राप्त सभी उपलब्ध कॉन्फ़िगरेशन विकल्पों की पूरी सूची के लिए, मुख्य FastComments कॉन्फ़िगरेशन दस्तावेज़ देखें।