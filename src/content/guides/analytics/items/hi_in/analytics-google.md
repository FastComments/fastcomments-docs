हम FastComments को Google Analytics 4 को सूचित करने के लिए कॉन्फ़िगर कर सकते हैं जब कोई कमेंट विजेट के साथ इंटरैक्ट करता है।

हम ट्रैक कर सकते हैं कि उपयोगकर्ता कब:

- कमेंट करते हैं।
- वोट करते हैं।

यह करने के लिए यहाँ एक उदाहरण कोड स्निपेट है:

[inline-code-attrs-start title = 'Google Analytics 4'; type = 'HTML'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        onReplySuccess: function (comment) {
            gtag('event', 'post_comment', {
                'event_category': 'Engagement',
                'event_label': 'Comment Posted'
            });
        },
        onVoteSuccess: function (comment) {
            gtag('event', 'vote_comment', {
                'event_category': 'Engagement',
                'event_label': 'User Voted on a Comment'
            });
        }
    }];
</script>
[inline-code-end]

यह दो इवेंट जोड़ेगा:

- लेबल: `Comment Posted`, श्रेणी: `Engagement`, ID: `post_comment`
- लेबल: `User Voted on a Comment`, श्रेणी: `Engagement`, ID: `vote_comment`