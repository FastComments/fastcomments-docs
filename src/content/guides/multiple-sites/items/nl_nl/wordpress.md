Each WordPress site needs its own FastComments account or tenant. WordPress numbers posts separately on each site, so post 5 on one site and post 5 on another would share a single comment thread if both were connected to the same account. For that reason, the plugin's setup refuses to connect a site when the account already has a different one connected.

[You can view your active integrations here.](https://fastcomments.com/auth/my-account/manage-data/integrations)

To run several WordPress sites, create a [tenant](https://fastcomments.com/auth/my-account/tenants) for each one, switch to it, and then connect the site from the plugin's setup page. Tenants are included in plans with white labeling. You can also create a separate FastComments account for each site, using a different email address for each.

If a site has moved, for example from staging to production, deactivate the plugin on the old site and remove the old site from your active integrations. Then connect the new one. Comments already synced from the old site stay on the account, and they show on any post with the same ID on the new site.

You can use the same FastComments account with a single WordPress installation that spans multiple domains and has the same content on those domains.