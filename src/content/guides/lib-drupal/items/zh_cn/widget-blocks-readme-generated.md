可以通过 **结构 > 区块布局** (`/admin/structure/block`) 使用多个区块：

- **FastComments Widget** - 主要的评论小部件。自动检测当前实体。跳过已经有 FastComments 字段的实体（以防重复）。
- **FastComments Live Chat** - 实时流式聊天。可以与同一页面上的评论字段并排放置。
- **FastComments Collab Chat** - 基于文本选择的注释与讨论。
- **FastComments Image Chat** - 基于坐标的图像注释。
- **FastComments Recent Comments** - 显示整个站点的最新评论。可配置的评论数量。
- **FastComments Top Pages** - 显示评论数最多的页面。

以内容为中心的区块（Live Chat、Collab Chat、Image Chat）会自动检测当前实体；在非实体页面上则回退为基于路径的标识符。