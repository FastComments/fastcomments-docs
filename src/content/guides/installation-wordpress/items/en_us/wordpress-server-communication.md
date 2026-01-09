For the plugin to work, a token is saved in your WordPress database and also your FastComments account. When the plugin makes a request to our servers, it provides
this token.

You can view all integrations authorized to your FastComments account [here](https://fastcomments.com/auth/my-account/manage-data/integrations).

All communication is done via HTTPS.

All communication is *outbound* from your WordPress server *to* FastComments.com, including the sync *back* to your WordPress installation as it is implemented
via [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) from a [cron](https://developer.wordpress.org/plugins/cron/) setup in your WordPress installation.

---