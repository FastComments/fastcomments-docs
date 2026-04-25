该模块包含多个可放置的区块，可从 `Structure > Block layout`（`/admin/structure/block`）中添加。

- **FastComments Widget** - 主要的评论小部件。会自动检测当前实体。对于已经附加了 FastComments 字段的实体，它会跳过，因此在同一页面上不会出现重复的小部件。
- **FastComments Live Chat** - 实时流式聊天。可以与同一页面上的评论字段并排放置。
- **FastComments Collab Chat** - 文本选择注释与讨论。
- **FastComments Image Chat** - 基于坐标的图片注释。访客点击图片以在特定位置留下评论。
- **FastComments Recent Comments** - 显示站点上的最新评论。数量可在区块中配置。
- **FastComments Top Pages** - 显示站点中评论最多的页面。

以内容为中心的区块（Live Chat、Collab Chat、Image Chat）会自动检测当前实体，在非实体页面上则回退到基于路径的标识符。这意味着它们在分类页面、视图和自定义路由上都能正常工作，无需额外设置。