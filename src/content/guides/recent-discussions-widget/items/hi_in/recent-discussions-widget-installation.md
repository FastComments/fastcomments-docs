Recent Discussions Widget आपकी साइट के उन पेजों को दिखाता है जिनमें सबसे हालिया कमेंट गतिविधि है। प्रत्येक एंट्री में पेज का शीर्षक, अंतिम गतिविधि की तारीख और कुल कमेंट संख्या दिखाई जाती है। यह स्वचालित रूप से डार्क बैकग्राउंड का पता लगाता है और अपनी स्टाइलिंग उसी के अनुसार समायोजित करता है।

## बेसिक इंस्टॉलेशन

[inline-code-attrs-start title = 'Recent Discussions Widget इंस्टॉलेशन'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## कॉन्फ़िगरेशन विकल्प

`FastCommentsRecentDiscussionsV2` फ़ंक्शन निम्नलिखित कॉन्फ़िगरेशन विकल्प स्वीकार करता है:

- **tenantId** (आवश्यक): आपकी FastComments tenant ID
- **count** (वैकल्पिक): दिखाने के लिए पेजों की संख्या। डिफ़ॉल्ट `20` है, अधिकतम `100`
- **hasDarkBackground** (वैकल्पिक): डार्क मोड स्टाइलिंग लागू करें। सेट न होने पर पेज के बैकग्राउंड से स्वतः पहचाना जाता है

## उन्नत उदाहरण

### कस्टम Count

[inline-code-attrs-start title = 'कस्टम Count के साथ Recent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### डार्क मोड लागू करें

[inline-code-attrs-start title = 'डार्क मोड के साथ Recent Discussions'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]