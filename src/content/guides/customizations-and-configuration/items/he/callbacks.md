כל הספריות עבור הווידג'ט התגובות (כרגע Angular, React, Vue) תומכות בקריאות-חזרה.

קריאות-החזרה מוגדרות באובייקט התצורה, עם אותה חתימה עבור כל ספרייה.

הקריאות-חזרה הנתמכות הן:

- onInit
- onAuthenticationChange
- onRender
- commentCountUpdated
- onReplySuccess
- onVoteSuccess
- onImageClicked
- onOpenProfile
- onCommentSubmitStart
- onCommentsRendered

החתימות המדויקות ניתן למצוא ב[הגדרות TypeScript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L124).

הנה דוגמה שבה כל הקריאות-חזרה מיושמות:

[inline-code-attrs-start title = 'דוגמאות של קריאות חזרה'; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo',
        onInit: function () {
            console.log('Library started to fetch comments!');
        },
        onAuthenticationChange: function (eventName, userObj) {
            console.log('User authenticated!', eventName, userObj);
        },
        onRender: function () {
            console.log('Render event happened!');
        },
        commentCountUpdated: function (newCount) {
            console.log('New comment count:', newCount);
        },
        onReplySuccess: function (comment) {
            console.log('New comment saved!', comment);
        },
        onVoteSuccess: function (comment, voteId, direction, status) {
            console.log('New vote saved!', comment, voteId, direction, status);
        },
        onImageClicked: function (src) {
            console.log('Image clicked!', src);
        },
        onOpenProfile: function (userId) {
            console.log('User tried to open profile', userId);
            // return true; // החזר true כדי למנוע את ההתנהגות המוגדרת כברירת מחדל (פתיחת פרופיל משתמש ב-fastcomments.com).
        },
        onCommentSubmitStart: function(comment, continueSubmitFn, cancelFn) {
            console.log('Trying to submit comment', comment);
            setTimeout(function() { // חיקוי התנהגות אסינכרונית (קריאת API וכו').
                if(confirm('Should submit?')) {
                    continueSubmitFn();
                } else {
                    cancelFn('Some optional error message');
                }
            }, 1000);
        },
        onCommentsRendered: function(comments) {
            // comments ממויין לפי המיון הדיפולטי בעמוד, אשר עשוי להיות 'Most Relevant' (למשל: עם הכי הרבה הצבעות, וכו'), או 'Newest First'
            const topCommentInList = comments[0];
            console.log('First Comment Rendered:', topCommentInList.avatarSrc, topCommentInList.commenterName, topCommentInList.commentHTML);
        }
    }];
</script>
[inline-code-end]