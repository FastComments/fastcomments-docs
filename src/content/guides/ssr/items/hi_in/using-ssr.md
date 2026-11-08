FastComments SSR का उपयोग करने के लिए, क्लाइंट `https://fastcomments.com/ssr/comments` एंडपॉइंट से HTML प्राप्त कर सकता है।

इसे कई तरीकों से किया जा सकता है।

### WordPress के साथ

WordPress प्लगइन में संस्करण `3.10.2` से, JS सक्षम न करने वाले उपयोगकर्ताओं के लिए SSR डिफ़ॉल्ट रूप से एक फॉलबैक के रूप में सक्षम है।

### एक वेबपेज में

पहले से मौजूद एप्लिकेशन के साथ, SSR को [निम्नलिखित उदाहरण](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr) के साथ जोड़ा जा सकता है, यह मानते हुए कि उपयोग की जाने वाली भाषा PHP है:

[inline-code-attrs-start title = 'PHP-आधारित SSR उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

हम SSR UI को केवल तब दिखा सकते हैं जब उपयोगकर्ता ने JS अक्षम किया हो:

[inline-code-attrs-start title = 'PHP-आधारित SSR फॉलबैक उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

SSO का उपयोग करने वाले उदाहरण के लिए, [यहाँ देखें](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr)।

### प्री-रेंडर की गई सामग्री के साथ

हमारा ब्लॉग बिल्ड समय पर जनरेट होता है, और [Handlebars के साथ SSR का एक अच्छा उदाहरण](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51) प्रदान करता है।

### मूल पैरामीटर

मूल पैरामीटर जिन्हें आपको पास करने की आवश्यकता है:
- `tenantId` - यह आपको एक ग्राहक के रूप में पहचानता है।
- `urlId` - यह उस पेज या आलेख की पहचान करता है जिसके लिए टिप्पणियाँ लोड करनी हैं, और यह निर्धारित करता है कि वे कहाँ सहेजी जाती हैं।
- `url` - इसका उपयोग सूचनाओं और संबंधित सुविधाओं के लिए टिप्पणी थ्रेड से लिंक करने के लिए किया जाता है।

### कस्टम स्टाइलिंग

टिप्पणी विजेट का SSR संस्करण JavaScript संस्करण के समान संरचना और रेंडरिंग इंजन का उपयोग करता है।

इसलिए, सभी कस्टम स्टाइलिंग जो JavaScript टिप्पणी विजेट के लिए काम करती है, वह SSR के लिए भी काम करती है।

### नोट्स

SSR के साथ, रेंडर किए गए कंटेनर की ऊँचाई को नियंत्रित करने के लिए कोई JavaScript नहीं होता है। ब्राउज़रों में, लंबी चर्चाओं के लिए एक वर्टिकल स्क्रॉलबार दिखाई दे सकता है।

इसलिए, आपको इसे अपनी इच्छानुसार समायोजित करना होगा।