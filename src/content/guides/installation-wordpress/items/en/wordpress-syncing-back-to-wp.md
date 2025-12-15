By default, FastComments syncs back to your WordPress site daily. This is purely for backup purposes so you continue to own a copy of the data, and for plugins
that may depend on it.

This does not happen immediately with every comment saved due to the nature of some sites being able to handle heavy read traffic, their database deployments are not always able to handle the heavy write traffic (hence offloading this work to FastComments).

The sync schedule back to the WordPress can be customed by installing a plugin. We recommend [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

The sync back to WordPress can also be performed any time manually by going to the FastComments plugin dashboard and selecting `Manually Sync`. You will have
the option to sync back to your WP install, or to re-upload your WP comments to FastComments servers.
