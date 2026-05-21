### 从 Shopify 应用商店安装

1. 打开 [FastComments 在 Shopify 应用商店的列表页](https://apps.shopify.com/fastcomments)。
2. 点击 **添加应用** 并在安装流程中选择您想要的方案。
3. 安装完成后，Shopify 会在安装完成后将您重定向回 Shopify 内的 FastComments 管理页面。

安装就是这么简单。无需将任何内容粘贴到您的主题文件中。

### 为您设置了什么

安装会自动完成您本来需要手动执行的所有操作：

- 为您的商店创建一个 FastComments 租户并将其链接到您的商店域名。
- 将您的商店 URL 添加到该租户的授权域名中，这样评论就可以在没有域名错误的情况下加载。
- 写入一个 `fastcomments.tenant_id` shop metafield，以便每个区块都知道要针对哪个租户进行渲染。
- 默认启用针对您的 Shopify 客户的单点登录。
- 计费通过 Shopify 托管定价运行。费用会显示在您常规的 Shopify 账单中。可在 Shopify 管理后台的 **设置 > 应用与销售渠道 > FastComments** 中升级、降级或取消。

如果您的商店在安装该应用之前已经是 FastComments 的客户，安装会重用现有租户而不是创建新的租户。

### 嵌入式管理面板

当您从 Shopify 管理后台打开 FastComments 应用时，您会进入一个包含一键式图块的仪表板，这些图块可以直接进入完整的 FastComments 后端：

- **Dashboard**：账户设置、使用情况和订阅详情。
- **Moderation Queue**：在整个商店中批准、拒绝和回复评论。
- **Customize**：调整小部件颜色、字体、审核规则和配置。
- **Ratings & Reviews Helper**：如果您想使用 Reviews Summary 区块，可设置星级评分和评论问题。

每个图块都会使用一次性登录链接打开 FastComments，因此您无需单独登录。

### 下一步：将区块添加到您的商店

打开您的 Shopify 主题编辑器（**在线商店 > 主题 > 自定义**），打开您想添加评论或评价的模板，然后点击 **添加区块**。FastComments 区块会显示在 **应用** 下。本指南的其余部分将介绍每一个区块。