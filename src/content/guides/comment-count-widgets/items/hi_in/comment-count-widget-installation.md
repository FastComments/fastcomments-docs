कमेंट काउंट विजेट किसी एक पेज की कमेंट संख्या दिखाने के लिए बनाया गया है। यह हल्का है और कॉन्फ़िगर किए जाने पर रीयल-टाइम अपडेट प्रदान करता है।

## बेसिक इंस्टॉलेशन

[inline-code-attrs-start title = 'कमेंट काउंट विजेट इंस्टॉलेशन'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## कॉन्फ़िगरेशन विकल्प

`FastCommentsCommentCount` फ़ंक्शन निम्नलिखित कॉन्फ़िगरेशन विकल्प स्वीकार करता है:

- **tenantId** (आवश्यक): आपकी FastComments टेनेंट ID
- **urlId** (वैकल्पिक): पेज पहचानकर्ता। यदि निर्दिष्ट नहीं किया गया है तो डिफ़ॉल्ट रूप से `window.location.href` होता है
- **numberOnly** (वैकल्पिक): यदि `true` है, तो केवल संख्या बिना किसी टेक्स्ट के रेंडर होती है। डिफ़ॉल्ट `false` है
- **isLive** (वैकल्पिक): यदि `true` है, तो संख्या स्वचालित रूप से अपडेट होगी। डिफ़ॉल्ट `false` है

## उन्नत उदाहरण

### कस्टम URL ID

[inline-code-attrs-start title = 'कस्टम URL ID के साथ कमेंट काउंट'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-custom"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-custom'), {
        tenantId: 'demo',
        urlId: 'my-custom-page-id'
    });
</script>
[inline-code-end]

### केवल संख्या डिस्प्ले

[inline-code-attrs-start title = 'कमेंट काउंट केवल संख्या'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-number"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-number'), {
        tenantId: 'demo',
        numberOnly: true
    });
</script>
[inline-code-end]

### लाइव अपडेट

[inline-code-attrs-start title = 'लाइव कमेंट काउंट अपडेट'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-live"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-live'), {
        tenantId: 'demo',
        isLive: true
    });
</script>
[inline-code-end]

## विजेट मेथड्स

विजेट निम्नलिखित मेथड्स के साथ एक ऑब्जेक्ट लौटाता है:

- **destroy()**: विजेट को हटाता है और सभी टाइमर साफ़ करता है
- **update(config)**: नई कॉन्फ़िगरेशन के साथ विजेट को अपडेट करता है

### उपयोग का उदाहरण

[inline-code-attrs-start title = 'विजेट मेथड्स उदाहरण'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-methods"></div>
<script>
    const widget = window.FastCommentsCommentCount(document.getElementById('comment-count-methods'), {
        tenantId: 'demo'
    });
    
    // किसी दूसरे पेज की संख्या दिखाने के लिए विजेट को अपडेट करें
    setTimeout(() => {
        widget.update({
            tenantId: 'demo',
            urlId: 'different-page-id'
        });
    }, 5000);
    
    // 10 सेकंड बाद विजेट को नष्ट करें
    setTimeout(() => {
        widget.destroy();
    }, 10000);
</script>
[inline-code-end]

## स्टाइलिंग

विजेट कमेंट संख्या के साथ सादा HTML रेंडर करता है और इसमें न्यूनतम स्टाइलिंग शामिल है। आप CSS के साथ उपस्थिति को अनुकूलित कर सकते हैं:

[inline-code-attrs-start title = 'कस्टम स्टाइलिंग'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<style>
    .comment-count-styled {
        background: #f0f0f0;
        padding: 5px 10px;
        border-radius: 15px;
        font-size: 14px;
        color: #666;
        display: inline-block;
    }
</style>
<script src="https://cdn.fastcomments.com/js/widget-comment-count.min.js"></script>
<div id="comment-count-styled" class="comment-count-styled"></div>
<script>
    window.FastCommentsCommentCount(document.getElementById('comment-count-styled'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]