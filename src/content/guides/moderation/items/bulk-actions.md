When moderating, sometimes there is a series of actions that we want to take all at once. For example maybe you find a
series of comments you want to delete, mark as spam, un-approve, or simply mark as reviewed. Instead of clicking "delete" and "confirm"
on each comment, there's a better way.

Next to each comment is a checkbox, which we can click to start a bulk action:

[app-screenshot-start url='/auth/my-account/moderate-comments?demo=true'; clickSelectors = ['.comment[data-comment-id="demo-0"] .checkmark', '.comment[data-comment-id="demo-1"] .checkmark', '.comment[data-comment-id="demo-2"] .checkmark']; selector = '.content'; title='Bulk Actions' app-screenshot-end]

Then we can pick a bulk action and confirm it:

[app-screenshot-start url='/auth/my-account/moderate-comments?demo=true'; clickSelectors = ['.comment[data-comment-id="demo-0"] .checkmark', '.comment[data-comment-id="demo-1"] .checkmark', '.comment[data-comment-id="demo-2"] .checkmark', '.bulk-actions .btn[data-indv-action="do-remove"]']; selector = '.content'; title='Bulk Removal' app-screenshot-end]

We will see the progress as our bulk actions are taken:

[app-screenshot-start url='/auth/my-account/moderate-comments?demo=true'; clickSelectors = ['.comment[data-comment-id="demo-0"] .checkmark', '.comment[data-comment-id="demo-1"] .checkmark', '.comment[data-comment-id="demo-2"] .checkmark', '.bulk-actions .btn[data-indv-action="do-remove"]', '.bulk-actions .bulk-action-prompt .bulk-action-confirm']; selector = '.content'; delay = 500; title='Bulk Removal In Progress' app-screenshot-end]
