### उपयोग के मामले

Image Chat डिज़ाइन फीडबैक के लिए बहुत अच्छा काम करता है, जहाँ टीमों को मॉकअप या स्क्रीनशॉट में विशिष्ट तत्वों पर चर्चा करनी होती है। उत्पाद समीक्षा साइटें ग्राहकों को उत्पाद की तस्वीरों में दिखाई देने वाली विशिष्ट विशेषताओं पर चर्चा करने दे सकती हैं। शैक्षिक प्लेटफ़ॉर्म इसका उपयोग आरेखों, नक्शों या वैज्ञानिक छवियों पर चर्चा के लिए कर सकते हैं। फोटो गैलरी स्थान-विशिष्ट टिप्पणियों के साथ इंटरैक्टिव बन सकती हैं। रियल एस्टेट साइटें दर्शकों को संपत्ति की तस्वीरों में दिखाई देने वाली विशिष्ट विशेषताओं पर चर्चा करने दे सकती हैं।

### त्वरित शुरुआत

Image Chat के साथ शुरुआत करना आसान है। आपको FastComments Image Chat स्क्रिप्ट, एक छवि वाला image element या container, और आपकी Tenant ID वाला एक configuration object चाहिए।

### इंस्टॉलेशन

Image Chat स्क्रिप्ट को अपने पेज में जोड़ें:

[inline-code-attrs-start title = 'Image Chat स्क्रिप्ट लोड करना'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### बुनियादी कार्यान्वयन

यहाँ एक न्यूनतम उदाहरण है:

[inline-code-attrs-start title = 'बुनियादी Image Chat कार्यान्वयन'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- आपकी छवि -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Image Chat स्क्रिप्ट लोड करें -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Image Chat प्रारंभ करें -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

यदि पहले से नहीं है, तो `'demo'` को अपनी वास्तविक FastComments Tenant ID से बदलें, जो आपको अपने [FastComments डैशबोर्ड](https://fastcomments.com/auth/my-account) में मिल सकती है।

### यह कैसे काम करता है

प्रारंभ होने के बाद, उपयोगकर्ता छवि पर कहीं भी क्लिक कर सकते हैं। जब कोई क्लिक होता है, तो उस स्थान पर एक दृश्य वर्गाकार मार्कर दिखाई देता है और एक चैट विंडो खुलती है। अन्य उपयोगकर्ता सभी मार्कर देख सकते हैं और उन पर क्लिक करके उन चर्चाओं को देख या उनमें भाग ले सकते हैं। सभी चर्चाएँ सभी आगंतुकों के बीच वास्तविक समय में सिंक होती हैं।

विजेट प्रतिशत-आधारित पोज़िशनिंग का उपयोग करता है, इसलिए छवि के आकार बदलने या विभिन्न स्क्रीन साइज़ पर देखे जाने पर भी मार्कर सही स्थान पर रहते हैं।

### लाइव डेमो

आप Image Chat को क्रिया में हमारे [लाइव डेमो पेज](https://fastcomments.com/product/image-chat) पर देख सकते हैं।

### अगले कदम

अब जब आपकी बुनियादी सेटअप काम कर रही है, आप Configuration Options गाइड में रूप और व्यवहार को अनुकूलित कर सकते हैं। प्रतिशत-आधारित पोज़िशनिंग कैसे काम करती है, यह समझने के लिए Responsive Design गाइड देखें। स्टाइलिंग और डार्क मोड समर्थन के बारे में Customization गाइड में जानें। उन्नत एकीकरणों के लिए, API Reference देखें।

### फ्रंटएंड लाइब्रेरी

सभी FastComments फ्रंटएंड लाइब्रेरी (react, vue, angular, आदि) में Image Chat है।