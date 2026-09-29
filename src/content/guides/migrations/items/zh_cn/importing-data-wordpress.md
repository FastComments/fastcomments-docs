我们的 [WordPress 插件](https://wordpress.org/plugins/fastcomments/) 拥有强大的基于 UI 的导入机制。安装插件后，
它会引导您将 WordPress 安装与 FastComments 关联，并复制您现有的评论数据。

**此过程无需手动复制或下载任何内容。**

迁移过程将在 UI 中向您指示。大多数迁移只需几分钟。

该机制旨在在迁移期间不对您的 WordPress 安装施加过大负载。

如果您要将站点从 WordPress 转移出去，可以导入 WordPress XML 或 CSV 导出文件，而不是使用插件。参见
[将您的评论迁移到新站点](/guide-installation-wordpress.html#wordpress-moving-off-wordpress)。

### CloudFlare 与防火墙

为了使自动化的 WordPress 设置生效，我们必须调用您的 WordPress 安装。
像 Cloudflare 这样的防火墙可能会阻止我们的请求，导致集成失败。在这种情况下，[我们可以为您提供](https://fastcomments.com/auth/my-account/help)一组需要列入白名单的 IP 地址，以完成集成。

### 数据所有权

在我们的 WordPress 迁移中，任何新建或更新的评论数据都会自动同步回您的 WordPress 安装，
在后台完成。这意味着，虽然评论由 FastComments 本身提供，以减轻您 WordPress 部署的负载，
我们 **也** 会将它们保存到您的数据库中作为备份。这同样意味着，如果您希望切换离开 FastComments，您的数据已经迁移并保持最新。