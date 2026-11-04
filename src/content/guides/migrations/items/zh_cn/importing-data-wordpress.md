我们的 [WordPress 插件](https://wordpress.org/plugins/fastcomments/) 拥有强大的基于 UI 的导入机制。安装插件后，它会引导您将 WordPress 安装与 FastComments 关联，并复制您现有的评论数据。

**整个过程无需手动复制或下载任何内容。**

迁移过程中会通过 UI 向您显示迁移进度。大多数迁移只需几分钟。

该机制的设计确保在迁移过程中不会给您的 WordPress 安装带来过大的负载。

如果您要将网站从 WordPress 迁出，可以导入 WordPress XML 或 CSV 导出文件，而无需使用插件。请参阅 [将您的评论迁移到新网站](/guide-installation-wordpress.html#wordpress-moving-off-wordpress)。

### CloudFlare 与防火墙

为了使自动化的 WordPress 设置正常运行，我们需要向您的 WordPress 安装发起请求。像 Cloudflare 这样的防火墙可能会拦截我们并导致集成失败。在这种情况下，[我们可以为您提供](https://fastcomments.com/auth/my-account/help) 一组需要加入白名单的 IP 地址，以便完成集成。

### 数据所有权

在我们的 WordPress 迁移中，任何新增或更新的评论数据都会在后台自动同步回您的 WordPress 安装。这意味着，虽然评论由 FastComments 本身提供服务以减轻您 WordPress 部署的负载，但我们**也**会将它们保存到您的数据库中作为备份。这也意味着，如果您希望不再使用 FastComments，您的数据已经迁移完毕并保持最新。