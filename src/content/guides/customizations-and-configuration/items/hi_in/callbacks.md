कमेंट विजेट के लिए सभी लाइब्रेरी (वर्तमान में Angular, React, Vue) कॉलबैक का समर्थन करती हैं।

कॉलबैक कॉन्फ़िगरेशन ऑब्जेक्ट में निर्दिष्ट किए जाते हैं, और प्रत्येक लाइब्रेरी के लिए उनका सिग्नेचर समान होता है।

समर्थित कॉलबैक ये हैं:

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

सटीक सिग्नेचर [TypeScript परिभाषाओं](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L124) में पाए जा सकते हैं।

सभी कॉलबैक के उपयोग का एक उदाहरण यहाँ दिया गया है:

[inline-code-attrs-start title = 'कॉलबैक उदाहरण'; inline-code-attrs-end]
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
            // return true; // डिफ़ॉल्ट व्यवहार (fastcomments.com यूज़र प्रोफ़ाइल खोलना) को रोकने के लिए true लौटाएँ।
        },
        onCommentSubmitStart: function(comment, continueSubmitFn, cancelFn) {
            console.log('Trying to submit comment', comment);
            setTimeout(function() { // असिंक्रोनस व्यवहार (API कॉल आदि) का अनुकरण।
                if(confirm('Should submit?')) {
                    continueSubmitFn();
                } else {
                    cancelFn('Some optional error message');
                }
            }, 1000);
        },
        onCommentsRendered: function(comments) {
            // comments पेज के डिफ़ॉल्ट सॉर्ट के अनुसार क्रमबद्ध हैं, जो Most Relevant (जैसे: सबसे अधिक अपवोट, आदि) या Newest First हो सकता है
            const topCommentInList = comments[0];
            console.log('First Comment Rendered:', topCommentInList.avatarSrc, topCommentInList.commenterName, topCommentInList.commentHTML);
        }
    }];
</script>
[inline-code-end]