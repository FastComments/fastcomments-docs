बल्क कमेंट काउंट विजेट एक ही पेज पर कई पेजों के लिए कमेंट काउंट कुशलतापूर्वक प्रदर्शित करने के लिए डिज़ाइन किया गया है। प्रत्येक कमेंट काउंट के लिए अलग-अलग API कॉल करने के बजाय, यह विजेट बेहतर प्रदर्शन के लिए अनुरोधों को बैच में भेजता है।

## बेसिक इंस्टॉलेशन

[inline-code-attrs-start title = 'बल्क कमेंट काउंट विजेट इंस्टॉलेशन'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## यह कैसे काम करता है

बल्क विजेट इस तरह काम करता है:

1. पेज पर `fast-comments-count` क्लास वाले एलिमेंट्स को स्कैन करता है
2. प्रत्येक एलिमेंट से `data-fast-comments-url-id` एट्रिब्यूट पढ़ता है
3. कई कमेंट काउंट कुशलतापूर्वक प्राप्त करने के लिए API अनुरोधों को बैच में भेजता है
4. प्रत्येक एलिमेंट को उपयुक्त कमेंट काउंट के साथ अपडेट करता है

## कॉन्फ़िगरेशन विकल्प

`FastCommentsCommentCountBulk` फ़ंक्शन निम्नलिखित कॉन्फ़िगरेशन विकल्प स्वीकार करता है:

- **tenantId** (आवश्यक): आपकी FastComments टेनेंट ID
- **apiHost** (वैकल्पिक): कस्टम API होस्ट, यदि आप सेल्फ-होस्टेड इंस्टेंस का उपयोग कर रहे हैं

## वास्तविक दुनिया का उदाहरण

यहाँ एक व्यावहारिक उदाहरण है जो दिखाता है कि आप ब्लॉग पोस्ट लिस्टिंग में बल्क विजेट का उपयोग कैसे कर सकते हैं:

[inline-code-attrs-start title = 'कमेंट काउंट के साथ ब्लॉग पोस्ट लिस्टिंग'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 | 
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 | 
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 | 
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## प्रदर्शन संबंधी विचार

बल्क विजेट स्वचालित रूप से प्रदर्शन को अनुकूलित करता है:

- **अनुरोधों का बैचिंग**: कई कमेंट काउंट एक ही API कॉल में प्राप्त किए जाते हैं
- **अनुरोध आकार सीमाएँ**: यदि URL सूची बहुत बड़ी हो जाती है (1,000 से अधिक वर्ण) तो अनुरोध स्वचालित रूप से विभाजित हो जाते हैं
- **डीप्लिकेशन**: समान `data-fast-comments-url-id` वाले कई एलिमेंट्स एक ही काउंट साझा करते हैं

## समान URL ID वाले कई एलिमेंट्स

आप पेज पर समान `data-fast-comments-url-id` वाले कई एलिमेंट्स रख सकते हैं। उन सभी को एक ही काउंट के साथ अपडेट किया जाएगा:

[inline-code-attrs-start title = 'समान URL ID वाले कई एलिमेंट्स'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## स्थानीयकरण

बल्क विजेट आपकी FastComments भाषा सेटिंग्स के आधार पर कमेंट काउंट को स्वचालित रूप से फ़ॉर्मेट करता है। यह निम्नलिखित के लिए उपयुक्त टेक्स्ट प्रदान करता है:

- शून्य कमेंट
- एक कमेंट  
- कई कमेंट

## बल्क बनाम सिंगल विजेट का उपयोग कब करें

**बल्क विजेट का उपयोग करें जब:**
- आपके पास एक ही पेज पर कई कमेंट काउंट हों
- आप कमेंट काउंट के साथ पोस्ट/आर्टिकल की सूची प्रदर्शित कर रहे हों
- प्रदर्शन महत्वपूर्ण हो (API कॉल कम करता है)

**सिंगल विजेट का उपयोग करें जब:**
- आपको पेज पर केवल एक कमेंट काउंट की आवश्यकता हो
- आपको लाइव अपडेट की आवश्यकता हो (सिंगल विजेट रियल-टाइम अपडेट का समर्थन करता है)
- आप अलग-अलग विजेट व्यवहार पर अधिक नियंत्रण चाहते हों