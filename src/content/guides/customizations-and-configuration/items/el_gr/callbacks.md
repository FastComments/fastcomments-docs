Όλες οι βιβλιοθήκες για το widget σχολίων (προς το παρόν Angular, React, Vue) υποστηρίζουν callbacks.

Τα callbacks ορίζονται στο αντικείμενο ρυθμίσεων, με την ίδια υπογραφή για κάθε βιβλιοθήκη.

Τα υποστηριζόμενα callbacks είναι:

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

Οι ακριβείς υπογραφές μπορούν να βρεθούν στους [Ορισμούς TypeScript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L124).

Ακολουθεί ένα παράδειγμα με όλα τα callbacks:

[inline-code-attrs-start title = 'Παραδείγματα Callbacks'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
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
            // return true; // επιστρέψτε true για να αποτρέψετε την προεπιλεγμένη συμπεριφορά (άνοιγμα προφίλ χρήστη στο fastcomments.com).
        },
        onCommentSubmitStart: function(comment, continueSubmitFn, cancelFn) {
            console.log('Trying to submit comment', comment);
            setTimeout(function() { // εξομοιώντας ασύγχρονη συμπεριφορά (κλήση API κ.λπ.).
                if(confirm('Should submit?')) {
                    continueSubmitFn();
                } else {
                    cancelFn('Some optional error message');
                }
            }, 1000);
        },
        onCommentsRendered: function(comments) {
            // τα σχόλια είναι ταξινομημένα με την προεπιλεγμένη ταξινόμηση στη σελίδα, η οποία μπορεί να είναι Most Relevant (π.χ. τα περισσότερα upvotes κ.λπ.) ή Newest First
            const topCommentInList = comments[0];
            console.log('First Comment Rendered:', topCommentInList.avatarSrc, topCommentInList.commenterName, topCommentInList.commentHTML);
        }
    });
</script>
[inline-code-end]