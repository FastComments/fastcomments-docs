Site Analytics, or just called `Analytics` in the dashboard, provides an overview of how your community is using FastComments across
all your domains.

FastComments provides some unique features that many other platforms do not offer, like **live** reporting on users online on each page,
and sorting pages by the number of online users. To do this, simply visit the [Analytics Page](https://fastcomments.com/auth/my-account/analytics) and
click `Sort by users online` under `Top Pages`.

Both the total `Users Online` and `Top Pages` metrics are live and are reported with no delay.

`Top Pages` by default will sort by the number of comments on each page.

Finally, a breakdown is provided for total metrics across your tenant, by day, over time for:

- Page Loads
  - This is the number of times a user opened a page that contains one or more FastComments widgets. If the page contains multiple widgets,
    then this number will be incremented by the number of widgets on that page. If you have an SPA, then every time the application opens a new
    comment thread, this number would be incremented. This applies to the React Native library as well.
  - This metric is also used for billing purposes in the Flex plans.
- Comments Left
  - This includes all comments, regardless of verification or approval state, or if they are spam or not.
- Votes Left
  - This is for the number of votes left. It will only count verified votes, unless anonymous voting is enabled.
- Accounts Created
  - This metric is for when a new SSO user is added, or a commenter comments with FastComments for the first time using your site.

These metrics are near-realtime, being delayed up to one minute.
