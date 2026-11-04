“最近讨论”小部件显示您网站上具有最新评论活动的页面。它适合突出仍在持续更新的讨论串，让访客可以重新加入活跃的对话，而不是停留在冷清的页面上。

## 选项

- **标题**（可选）：显示在列表上方的标题。默认为“最近讨论”。
- **数量**（可选）：显示多少条讨论。范围为 1 到 50。默认为 20。

## 如何添加

### 在文章或页面内

在区块编辑器中，添加一个**短代码**区块并粘贴：

[inline-code-attrs-start title = '最近讨论短代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

`count` 属性接受 1 到 50 之间的任意值。

### 在侧边栏或页脚中（经典主题）

在 WordPress 管理后台中前往**外观 > 小部件**。在区块插入器中搜索“FastComments”，并选择 **FastComments: Recent Discussions**。将其拖入侧边栏、页眉或页脚区域，然后在小部件面板中配置标题和数量。

### 在区块主题中（全站编辑）

打开**外观 > 编辑器**下的**站点编辑器**。导航到小部件应出现的模板部分，插入一个**旧版小部件**区块，并从下拉菜单中选择 **FastComments: Recent Discussions**。

## 故障排除

只有在 FastComments 设置完成并存储了租户 ID 后，小部件才会渲染。如果小部件区域为空白，请在 WordPress 管理后台的 **FastComments** 下完成设置，然后重新加载页面。

如果讨论排序看起来未更新，请检查底层页面是否已在 FastComments 控制面板中完成同步。小部件读取的是实时数据，因此新导入的评论可能需要一点时间才会显示。