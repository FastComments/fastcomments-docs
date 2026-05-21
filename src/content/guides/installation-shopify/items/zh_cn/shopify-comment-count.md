The **FastComments - Comment Count** block 渲染单个页面的小型评论计数。将其用于博客文章列表、产品卡片或任何链接到带有评论的页面的模板，以便访客在点击进入之前即可看到每个主题的活跃程度。

### 添加区块

1. 打开 Shopify 主题编辑器。
2. 打开要显示计数的模板。例如，**Blog** 模板（文章列表）或产品列表部分。
3. 在呈现每个项目的部分中点击 **Add block**。
4. 在 **Apps** 下，选择 **FastComments - Comment Count**。
5. 点击 **Save**。

### 设置

| 设置 | 功能 | 默认值 |
|---|---|---|
| 租户 ID（可选） | 覆盖计数读取的 FastComments 租户。留空以使用商店自动配置的租户。 | (空白) |
| 自定义 URL ID | 覆盖计数查找的页面标识符。当计数位于与其跟踪的 FastComments 区块不同的页面时使用此项。 | (自动检测) |

### 计数如何匹配评论主题

Comment Count block 使用与 **FastComments** 区块相同的自动检测逻辑：

- 博客文章模板： `shopify-article-{article.id}`
- 产品模板： `shopify-product-{product.id}`
- 其他模板：请求路径

如果您在页面上的 **FastComments** 区块上设置了 **Custom URL ID**，请在 Comment Count block 上设置相同的 Custom URL ID，以便它们指向相同的主题。

### 提示

- 页面上每个项目的计数会在一次请求中获取，因此在长列表的每个项目中添加该区块不会产生额外的往返成本。
- 在列表中，每篇文章或每个产品使用一个 Comment Count block 是预期的用法；该区块可根据需要多次添加。