The **FastComments - 评论摘要** 区块显示页面的聚合星级评分和评论细分。将其与产品模板上的 **FastComments** 区块配对以获得标准的评论布局：顶部摘要，下面是评论表单和评论列表。

### 前提条件：设置 评分与评论

评论摘要区块会显示您为商店配置的评分问题。请先进行以下设置：

1. 在 Shopify 管理后台打开 FastComments 应用。
2. 点击 **评分与评论助手** 磁贴（或直接打开 [评分与评论助手](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify)）。
3. 添加您希望每位评论者回答的问题（总体星级评分、“尺码如何”等）。

如果未配置问题，摘要区块将没有可聚合的数据。

### 添加区块

1. 打开 Shopify 主题编辑器。
2. 打开 **Product** 模板（或您希望放置摘要的页面模板）。
3. 在页面区段顶部靠近 **FastComments** 区块上方点击 **添加区块**。
4. 在 **应用** 下，选择 **FastComments - 评论摘要**。
5. 如果尚未添加，请在同一页面稍下方添加一个 **FastComments** 区块，以便访客可以留下评论。
6. 点击 **保存**。

### 设置

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | 覆盖摘要读取的 FastComments 租户。留空以使用商店自动配置的租户。 | (blank) |
| Custom URL ID | 覆盖摘要用于聚合的页面标识符。当摘要位于与其所反映的 FastComments 区块不同的页面上时使用此项。 | (auto-detected) |

### 摘要如何与评论匹配

评论摘要区块使用与 **FastComments** 区块相同的自动检测逻辑：

- 产品模板： `shopify-product-{product.id}`
- 博客文章模板： `shopify-article-{article.id}`
- 其他模板：请求路径

对于普通的产品页面，摘要和评论线程会自动共享 URL ID，无需任何配置。

### 提示

- 摘要为只读。要收集评论，您需要在同一页面上放置一个 **FastComments** 区块。
- 如果在收集评论后在 评分与评论助手 中更改评分问题，摘要会根据新的问题集重新计算。