最新评论 小部件显示整个站点发布的最新评论。它适用于侧边栏、页脚，或任何你想展示最新活动以鼓励进一步阅读的地方。

## 选项

- **Title**（可选）：显示在列表上方的标题。默认为 "最新评论"。
- **Count**（可选）：要显示的评论数量。范围 1 到 50。默认为 5。

## 如何添加

### 在文章或页面内

在区块编辑器中，添加一个 **Shortcode** 区块并粘贴：

[inline-code-attrs-start title = '最新评论 短代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

`count` 属性接受介于 1 到 50 之间的任何值。

### 在侧边栏或页脚（经典主题）

在你的 WordPress 管理后台，转到 **外观 > 小工具**。在区块插入器中搜索 "FastComments"，并选择 **FastComments: Recent Comments**。将其拖到侧边栏、页眉或页脚区域，然后在小工具面板中配置标题和数量。

### 在区块主题（全站编辑）中

在 **外观 > 编辑器** 下打开 **站点编辑器**。导航到应显示该小部件的模板部分，插入一个 **Legacy Widget** 区块，并从下拉菜单中选择 **FastComments: Recent Comments**。

## 故障排除

仅在完成 FastComments 设置并存储租户 ID 后，该小部件才会渲染。如果小部件区域为空，请在 WordPress 管理后台的 **FastComments** 中完成设置并重新加载页面。

---