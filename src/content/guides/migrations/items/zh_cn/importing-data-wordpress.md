Our [WordPress 插件](https://wordpress.org/plugins/fastcomments/) has a powerful UI-based importing mechanism. Upon installing the plugin,
it will guide you through linking your WordPress installation with FastComments and copying your existing comment data over.

**这将在不手动复制或下载任何内容的情况下完成。**

The migration process will be indicated to you via the UI during the migration. Most migrations only take a couple of minutes.

The mechanism is designed to not put excessive load on your WordPress installation during the migration.

### CloudFlare 与防火墙

In order for the automated WordPress setup to work, we have to make calls to your WordPress installation.
Firewalls like Cloudflare may block us and cause the integration to fail. In such cases, [我们可以为您提供](https://fastcomments.com/auth/my-account/help) with a set of IPs to whitelist for the integration.

### 数据所有权

In the case of our WordPress migration, any new or updated comment data is automatically synced back to your WordPress installation
behind the scenes. This means that, while the comments are served by FastComments itself to take load off of your WordPress deployment,
we **also** save them in your database as a backup. This also means if you desire to switch away from FastComments, your data is
already migrated and up to date.