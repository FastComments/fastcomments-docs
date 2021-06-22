All libraries for the comment widget (currently Angular, React, Vue) support callbacks.

The callbacks are specified in the configuration object, with the same signature for each library.

The callbacks supported are:

- onInit
- onRender
- onReplySuccess
- onVoteSuccess
- onImageClicked

The exact signatures can be found in the [TypeScript definitions](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L77).

Here's an example with all callbacks used:

[inline-code-attrs-start title = 'Callbacks Examples'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: 'demo',
        onInit: function () {
            console.log('Library started to fetch comments!');
        },
        onRender: function () {
            console.log('Render event happened!');
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
    });
</script>
[inline-code-end]

