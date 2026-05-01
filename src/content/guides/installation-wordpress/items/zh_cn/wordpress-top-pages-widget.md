Top Pages 小部件显示网站上评论最多的页面。它有助于向新访客展示互动性最高的内容并增加网站停留时间。

## 选项

- **标题**（可选）：显示在列表上方的标题。默认为 "Top Pages"。

Top Pages 小部件会根据可用数据自动选择布局，并且不接受 count 属性。

## 如何添加

### 在文章或页面内

在区块编辑器中，添加一个 **Shortcode** 区块并粘贴：

[inline-code-attrs-start title = 'Top Pages 简码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### 在侧边栏或页脚（经典主题）

在你的 WordPress 管理后台，转到 **外观 > 小工具**。在区块插入器中搜索 "FastComments"，并选择 **FastComments: Top Pages**。将其拖入侧边栏、页眉或页脚区域，然后在小工具面板中设置标题。

### 在区块主题（全站编辑）

在 **外观 > 编辑器** 下打开 **Site Editor**。导航到小部件应出现的模板部分，插入一个 **Legacy Widget** 区块，并从下拉菜单中选择 **FastComments: Top Pages**。

## 故障排除

只有在完成 FastComments 设置并存储 tenant ID 后，小部件才会渲染。如果小部件区域为空，请在 WordPress 管理后台的 **FastComments** 中完成设置并重新加载页面。