### त्वरित शुरुआत

Collab Chat के साथ शुरुआत करना आसान है। आपको FastComments Collab Chat स्क्रिप्ट, एक HTML एलिमेंट जिसमें वह टेक्स्ट हो जिसे आप एनोटेट करना चाहते हैं, और अपनी Tenant ID के साथ एक कॉन्फ़िगरेशन ऑब्जेक्ट की आवश्यकता है।

### इंस्टॉलेशन

अपने पेज में Collab Chat स्क्रिप्ट जोड़ें:

[inline-code-attrs-start title = 'Collab Chat स्क्रिप्ट लोड करना'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### बेसिक इम्प्लीमेंटेशन

यहाँ एक न्यूनतम उदाहरण है:

[inline-code-attrs-start title = 'बेसिक Collab Chat इम्प्लीमेंटेशन'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Your content container -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Initialize Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

यदि पहले से नहीं है, तो `'demo'` को अपनी वास्तविक FastComments Tenant ID से बदलें, जिसे आप अपने [FastComments डैशबोर्ड](https://fastcomments.com/auth/my-account/api-secret) में पा सकते हैं।

### यह कैसे काम करता है

इनिशियलाइज़ होने के बाद, उपयोगकर्ता टारगेट एलिमेंट के भीतर कोई भी टेक्स्ट चुन सकते हैं। एक छोटे विलंब (डेस्कटॉप पर 3.5 सेकंड) के बाद, एक प्रॉम्प्ट दिखाई देता है जो उन्हें चर्चा शुरू करने की अनुमति देता है। जब कोई चर्चा बनाई जाती है, तो टेक्स्ट पर एक विज़ुअल हाइलाइट दिखाई देता है। अन्य उपयोगकर्ता हाइलाइट पर होवर करके या क्लिक करके चर्चा देख सकते हैं और उसमें भाग ले सकते हैं। सभी चर्चाएँ सभी विज़िटर्स के लिए रियल-टाइम में सिंक होती हैं।

### लाइव डेमो

आप Collab Chat को क्रिया में हमारे [लाइव डेमो पेज](https://fastcomments.com/product/collab-chat) पर देख सकते हैं।

### अगले कदम

अब जब आपकी बेसिक्स काम कर रही हैं, तो आप Configuration Options गाइड में दिखावट और व्यवहार को कस्टमाइज़ कर सकते हैं। टेक्स्ट सिलेक्शन कैसे काम करता है, यह समझने के लिए Text Selection Behavior गाइड देखें। Customization गाइड में स्टाइलिंग और डार्क मोड सपोर्ट के बारे में जानें। उन्नत इंटीग्रेशन के लिए, API Reference देखें।

### फ्रंटएंड लाइब्रेरी

सभी FastComments फ्रंटएंड लाइब्रेरी (react, vue, angular, आदि) में Collab Chat है।