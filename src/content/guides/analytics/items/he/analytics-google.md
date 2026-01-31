---
אפשר להגדיר את FastComments כדי להודיע ל־גוגל אנליטיקס 4 כאשר מישהו מתקשר עם הווידג'ט של התגובות.

אפשר לעקוב מתי משתמשים:

- תגובה.
- הצבעה.

להלן דוגמת קוד לכך:

[inline-code-attrs-start title = 'גוגל אנליטיקס 4'; type = 'HTML'; isFunctional = false; inline-code-attrs-end]
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

זה יוסיף שני אירועים:

- תווית: `Comment Posted`, קטגוריה: `Engagement`, מזהה: `post_comment`
- תווית: `User Voted on a Comment`, קטגוריה: `Engagement`, מזהה: `vote_comment`

---