אנחנו יכולים להגדיר את FastComments להודיע ל-Google Analytics 4 כאשר מישהו מתקשר עם וידג'ט התגובות.

אנחנו יכולים לעקוב כאשר משתמשים:

- מגיבים.
- מצביעים.

הנה דוגמת קוד לכך:

[inline-code-attrs-start title = 'Google Analytics 4'; type = 'HTML'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
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
    });
</script>
[inline-code-end]

זה יוסיף שני אירועים:

- תווית: `Comment Posted`, קטגוריה: `Engagement`, מזהה: `post_comment`
- תווית: `User Voted on a Comment`, קטגוריה: `Engagement`, מזהה: `vote_comment`
