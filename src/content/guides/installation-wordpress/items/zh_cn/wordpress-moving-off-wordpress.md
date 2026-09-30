如果您要将站点从 WordPress 迁移并希望在新站点上使用 FastComments，则无需 WordPress 插件。请从 WordPress 导出您的评论，然后在 FastComments 仪表板的 [导入页面](https://fastcomments.com/auth/my-account/manage-data/import) 上传文件。

我们支持两种 WordPress 导出格式。

### WordPress XML（推荐）

这是 WordPress 内置导出器生成的文件，因此不需要额外的插件。

1. 在您的 WordPress 管理后台，前往 `Tools -> Export`。
2. 选择 `All content` 并点击 `Download Export File`。
3. 在 FastComments 的 [导入页面](https://fastcomments.com/auth/my-account/manage-data/import) 中，选择 `WordPress (.xml)` 并上传文件。

每条评论都关联到其所在文章的 URL，该信息已包含在文件中。

导入会保留作者姓名、电子邮件和网站、日期、内容、回复线程以及评论是否已批准。评论者头像会从 Gravatar 带入。投票不在此格式中。

### WordPress CSV

该文件来源于 [WebToffee 的 WordPress 评论导入导出插件](https://wordpress.org/plugins/comments-import-export-woocommerce/)。

1. 在您的 WordPress 管理后台安装该插件，并将评论导出为 CSV。
2. 将每个 `comment_post_ID` 的值替换为文章的 URL。
3. 在 FastComments 的 [导入页面](https://fastcomments.com/auth/my-account/manage-data/import) 中，选择 `WordPress (.csv)` 并上传文件。

每条评论都关联到 `comment_post_ID` 列。WordPress 用文章 ID 填充此列，而您的新站点没有 WordPress 文章 ID，因此第 2 步将其替换为 URL。

导入会保留作者姓名、电子邮件和网站、日期、内容、回复线程以及评论是否已批准。评论者头像会从 Gravatar 带入。如果文件中包含这些信息，还会保留 WordPress 的垃圾评论标记以及 wpDiscuz 的赞踩。

### 将评论匹配到新页面

如果您的新站点保留了与 WordPress 站点相同的 URL，评论会自动显示在对应页面，无需额外设置。

如果域名发生更改，请在导入后运行 [域名迁移工具](/guide-migrations.html#migrating-domains)。如果单个页面的 URL 变化，您可以 [迁移每个页面](/guide-migrations.html#migrating-pages) 从旧 URL 到新 URL。

对于批量页面迁移，例如从传递给评论小部件的 [urlId](/guide-customizations-and-configuration.html#url-id) 字段的值中移除域名，请 [打开支持工单](https://fastcomments.com/auth/my-account/help)，我们将为您处理。

### 切换前准备

您可以随意多次运行导入。重新导入相同文件 [不会产生重复](/guide-migrations.html#importing-data)，因此您可以先导入一次以测试新站点，然后在切换前再次导入最新的评论。

对于大于 1GB 的导出文件，请 [联系支持](https://fastcomments.com/auth/my-account/help)。

要将 FastComments 添加到您的新站点，请参阅 [安装指南](/guide-installation.html)。