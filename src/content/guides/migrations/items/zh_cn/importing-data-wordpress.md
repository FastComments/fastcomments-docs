---
我们的 [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) 具有强大的基于 UI 的导入机制。安装插件后，它会引导您将 WordPress 安装与 FastComments 关联并复制现有的评论数据。

**此过程无需手动复制或下载任何内容。**

迁移过程将在迁移期间通过 UI 向您显示。大多数迁移只需几分钟。

该机制在迁移期间设计为不会对您的 WordPress 安装造成过重负载。

### CloudFlare & 防火墙

为了使自动 WordPress 设置正常工作，我们必须向您的 WordPress 安装发出请求。像 Cloudflare 这样的防火墙可能会阻止我们并导致集成失败。在这种情况下，[我们可以向您提供](https://fastcomments.com/auth/my-account/help) 一组需加入白名单的 IP 以完成集成。

### 数据所有权

就我们的 WordPress 迁移而言，任何新的或已更新的评论数据都会在后台自动同步回您的 WordPress 安装。这意味着，尽管评论由 FastComments 本身提供以减轻您 WordPress 部署的负载，我们 **也** 会将它们保存在您的数据库中作为备份。这也意味着如果您希望切换离开 FastComments，您的数据已经被迁移并且是最新的。

---