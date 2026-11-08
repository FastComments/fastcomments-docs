### मूल उदाहरण

Image Chat का उपयोग करने का सबसे सरल तरीका एक single image element को target करना है। यह उदाहरण दिखाता है कि किसी image पर interactive discussions को कैसे सक्षम किया जाए:

[inline-code-attrs-start title = 'मूल Image Chat उदाहरण'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### Container Element के साथ उदाहरण

आप एक container element भी pass कर सकते हैं जिसके अंदर एक image हो:

[inline-code-attrs-start title = 'Container के साथ Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### Custom URL ID के साथ उदाहरण

डिफ़ॉल्ट रूप से, Image Chat conversations की पहचान करने के लिए page URL को image source और coordinates के साथ जोड़ता है। आप एक custom `urlId` प्रदान कर सकते हैं:

[inline-code-attrs-start title = 'Custom URL ID के साथ Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

यह तब उपयोगी है जब आपकी URL संरचना बदल जाती है लेकिन आप उन्हीं conversations को बनाए रखना चाहते हैं, या जब आप एक ही discussion points को कई pages पर share करना चाहते हैं।

### Dark Mode के साथ उदाहरण

यदि आपकी site का background dark है और widget इसे जैसा होना चाहिए वैसा auto detect नहीं कर पा रहा है, तो हम dark mode support को manually सक्षम कर सकते हैं:

[inline-code-attrs-start title = 'Dark Mode के साथ Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Custom Chat Square Size के साथ उदाहरण

आप image पर दिखाई देने वाले clickable squares का आकार समायोजित कर सकते हैं। आकार image की width के percentage के रूप में निर्दिष्ट किया जाता है:

[inline-code-attrs-start title = 'Custom Square Size के साथ Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // छोटे squares (डिफ़ॉल्ट 5 है)
        });
    </script>
</body>
</html>
[inline-code-end]

### Comment Count Callback के साथ उदाहरण

`commentCountUpdated` callback का उपयोग करके track करें कि comments कब add या update होते हैं:

[inline-code-attrs-start title = 'Comment Count Callback के साथ Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### Multiple Images के साथ उदाहरण

आप कई images पर Image Chat initialize कर सकते हैं। प्रत्येक image के अपने स्वतंत्र discussion points होंगे:

[inline-code-attrs-start title = 'Multiple Images पर Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // पहली image पर initialize करें
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // दूसरी image पर initialize करें
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]