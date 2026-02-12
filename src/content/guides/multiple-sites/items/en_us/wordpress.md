You can use the same FastComments account with multiple WordPress deployments. When you
set up the plugin, a separate integration token is added each time. [You can view your active integrations here.](https://fastcomments.com/auth/my-account/manage-data/integrations)

Since WordPress uses numeric page IDs (e.g. `?p=123`), you should create a separate tenant in your FastComments account for each
WordPress site. This ensures comments stay separated between sites, since different sites may have overlapping page IDs.
You can create additional tenants from [your account page](https://fastcomments.com/auth/my-account).

You can also use the same FastComments account with a WordPress installation that spans multiple domains and
has the same content on those domains.