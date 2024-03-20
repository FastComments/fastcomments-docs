We can configure FastComments to notify Google Analytics 4 when someone interacts with the comment widget.

We can track when users:

- Comment.
- Vote.

Here's an example code snippet to do that:

[code-example-start config = {onReplySuccess: function (comment) {gtag('event', 'post_comment', {'event_category': 'Engagement', 'event_label': 'Comment Posted'});}, onVoteSuccess: function (comment) {gtag('event', 'vote_comment', {'event_category': 'Engagement', 'event_label': 'User Voted on a Comment'});}}; linesToHighlight = [6, 7, 8, 9, 10, 11]; title = 'Google Analytics 4'; code-example-end]

This will add two events:

- Label: `Comment Posted`, Category: `Engagement`, ID: `post_comment`
- Label: `User Voted on a Comment`, Category: `Engagement`, ID: `vote_comment`
