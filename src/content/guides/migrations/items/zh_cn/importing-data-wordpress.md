Our [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) has a powerful UI-based importing mechanism. Upon installing the plugin,
it will guide you through linking your WordPress installation with FastComments and copying your existing comment data over.

**此过程无需手动复制或下载任何内容。**

The migration process will be indicated to you via the UI during the migration. Most migrations only take a couple of minutes.

The mechanism is designed to not put excessive load on your WordPress installation during the migration.

If you are moving your site off of WordPress, you can import a WordPress XML or CSV export instead of using the plugin. See
[将您的评论迁移到新站点](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare 与防火墙

In order for the automated WordPress setup to work, we have to make calls to your WordPress installation.
Firewalls like Cloudflare may block us and cause the integration to fail. In such cases, [我们可以为您提供](https://fastcomments.com/auth/my-account/help) with a set of IPs to whitelist for the integration.

### 数据所有权

In the case of our WordPress migration, any new or updated comment data is automatically synced back to your WordPress installation
behind the scenes. This means that, while the comments are served by FastComments itself to take load off of your WordPress deployment,
we **also** save them in your database as a backup. This also means if you desire to switch away from FastComments, your data is
already migrated and up to date.