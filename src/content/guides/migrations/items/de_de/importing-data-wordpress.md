Our [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) has a powerful UI-based importing mechanism. Upon installing the plugin,
it will guide you through linking your WordPress installation with FastComments and copying your existing comment data over.

**Dies geschieht, ohne etwas manuell zu kopieren oder herunterzuladen.**

The migration process will be indicated to you via the UI during the migration. Most migrations only take a couple of minutes.

The mechanism is designed to not put excessive load on your WordPress installation during the migration.

If you are moving your site off of WordPress, you can import a WordPress XML or CSV export instead of using the plugin. See
[Verschieben Ihrer Kommentare zu einer neuen Seite](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & Firewalls

In order for the automated WordPress setup to work, we have to make calls to your WordPress installation.
Firewalls like Cloudflare may block us and cause the integration to fail. In such cases, [können wir Ihnen](https://fastcomments.com/auth/my-account/help) with a set of IPs to whitelist for the integration.

### Datenbesitz

In the case of our WordPress migration, any new or updated comment data is automatically synced back to your WordPress installation
behind the scenes. This means that, while the comments are served by FastComments itself to take load off of your WordPress deployment,
we **also** save them in your database as a backup. This also means if you desire to switch away from FastComments, your data is
already migrated and up to date.