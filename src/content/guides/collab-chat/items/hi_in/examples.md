### बेसिक उदाहरण

Collab Chat का उपयोग करने का सबसे सरल तरीका एक सिंगल कंटेंट कंटेनर को टारगेट करना है। यह उदाहरण दिखाता है कि एक आर्टिकल पर टेक्स्ट एनोटेशन कैसे सक्षम करें:

[inline-code-attrs-start title = 'बेसिक Collab Chat उदाहरण'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### कस्टम URL ID के साथ उदाहरण (प्रति बुक पेज, आर्टिकल, आदि)

डिफ़ॉल्ट रूप से, Collab Chat बातचीत की पहचान करने के लिए पेज URL को एलिमेंट पाथ और टेक्स्ट रेंज के साथ जोड़ता है। आप बातचीत को ग्रुप करने के तरीके पर अधिक नियंत्रण पाने के लिए एक कस्टम `urlId` प्रदान कर सकते हैं:

[inline-code-attrs-start title = 'कस्टम URL ID के साथ Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        urlId: 'book-one-page-2'
    });
</script>
[inline-code-end]

यह तब उपयोगी है जब आपकी URL संरचना बदल जाती है लेकिन आप उन्हीं बातचीत को बनाए रखना चाहते हैं, या जब आप एक ही बातचीत के एनोटेशन को कई पेजों पर साझा करना चाहते हैं।

### डार्क मोड के साथ उदाहरण

यदि आपकी साइट का बैकग्राउंड डार्क है, तो यह सुनिश्चित करने के लिए डार्क मोड सपोर्ट सक्षम करें कि चैट UI सही दिखे:

[inline-code-attrs-start title = 'डार्क मोड के साथ Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat - Dark Mode</title>
    <style>
        body {
            background-color: #1a1a1a !important;
            color: #e0e0e0 !important;
            font-family: system-ui, -apple-system, sans-serif;
            padding: 20px;
            margin: 0;
        }
        #article-content {
            max-width: 800px;
            margin: 0 auto;
            background-color: #2d2d2d;
            padding: 20px;
            border-radius: 8px;
        }
        h1 {
            color: #ffffff !important;
        }
        p {
            color: #e0e0e0 !important;
            line-height: 1.6;
        }
        .fastcomments-collab-chat-top-bar {
            background-color: #2d2d2d !important;
            color: #e0e0e0 !important;
            border-bottom: 1px solid #444 !important;
        }
    </style>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo',
            hasDarkBackground: true
        });
    </script>
</body>
</html>
[inline-code-end]

### टॉप बार अक्षम के साथ उदाहरण

डिफ़ॉल्ट रूप से, Collab Chat यूज़र संख्या और चर्चा संख्या के साथ एक टॉप बार दिखाता है। आप इसे अक्षम कर सकते हैं:

[inline-code-attrs-start title = 'टॉप बार अक्षम के साथ Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat - No Top Bar</title>
</head>
<body>
    <div id="article-content" style="min-height: 500px">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo',
            topBarTarget: null
        });
    </script>
</body>
</html>
[inline-code-end]

### कमेंट काउंट कॉलबैक के साथ उदाहरण

आप `commentCountUpdated` कॉलबैक का उपयोग करके ट्रैक कर सकते हैं कि कमेंट कब जोड़े या अपडेट किए जाते हैं:

[inline-code-attrs-start title = 'कमेंट काउंट कॉलबैक के साथ Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    FastCommentsCollabChat(document.getElementById('article-content'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### एकाधिक सेक्शन के साथ उदाहरण

आप अपने पेज के कई अलग-अलग सेक्शन पर Collab Chat इनिशियलाइज़ कर सकते हैं। प्रत्येक सेक्शन के अपने स्वतंत्र एनोटेशन होंगे:

[inline-code-attrs-start title = 'एकाधिक सेक्शन पर Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div id="intro-section">
    <h2>Introduction</h2>
    <p>Content for the introduction...</p>
</div>

<div id="main-section">
    <h2>Main Content</h2>
    <p>Content for the main article...</p>
</div>

<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    // इंट्रो सेक्शन पर इनिशियलाइज़ करें
    FastCommentsCollabChat(document.getElementById('intro-section'), {
        tenantId: 'demo',
        urlId: 'my-article-intro'
    });

    // मुख्य सेक्शन पर इनिशियलाइज़ करें
    FastCommentsCollabChat(document.getElementById('main-section'), {
        tenantId: 'demo',
        urlId: 'my-article-main'
    });
</script>
[inline-code-end]