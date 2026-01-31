Μπορούμε να ρυθμίσουμε το FastComments ώστε να ειδοποιεί το Google Analytics 4 όταν κάποιος αλληλεπιδρά με το widget σχολίων.

Μπορούμε να παρακολουθούμε πότε οι χρήστες:

- Σχολιάζουν.
- Ψηφίζουν.

Ακολουθεί ένα παράδειγμα κώδικα για αυτό:

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

Αυτό θα προσθέσει δύο συμβάντα:

- Ετικέτα: `Comment Posted`, Κατηγορία: `Engagement`, ID: `post_comment`
- Ετικέτα: `User Voted on a Comment`, Κατηγορία: `Engagement`, ID: `vote_comment`