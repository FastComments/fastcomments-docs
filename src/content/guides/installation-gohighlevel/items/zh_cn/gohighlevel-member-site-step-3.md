现在我们将为你生成自定义的 FastComments 代码。使用下面的向导来配置 FastComments 在你的 GoHighLevel 网站上的工作方式：

[snippet id="gohighlevel-wizard"]

### 不同的评论框类型

你可以配置 `TYPE = 'commenting'` 这一行来切换使用的产品（例如你可以将其更改为 `live` 用于流式聊天，或 `collab` 用于协作聊天）。

### 将评论框放置在你想要的位置

假设你想在页面的特定部分放置评论框，而不是默认位置。
更改这行：

    const TARGET_ELEMENT_ID = ''; // 设置为使用目标 div 模式

为：

    const TARGET_ELEMENT_ID = 'fc_box'; // 设置为使用目标 div 模式

然后在 GHL 编辑器中，点击“code”按钮并添加你想放置评论的位置：

[inline-code-attrs-start title = 'GoHighLevel FastComments Div 元素'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### 每页不同的评论框类型

假设你希望用户高亮并讨论文本片段，或者改用流式聊天界面。

首先按照上面“将评论框放置在你想要的位置”中的步骤操作。

注意在上面的那段小片段中有 `type="commenting"`。

例如，如果你想启用协作聊天，请将 type 改为 `type="collab"`。

### 仅在特定页面显示

如果你不设置 不要设置 `TARGET_ELEMENT_ID`，你可以改为配置 `VALID_PATTERNS` 变量，以设置评论应该显示哪些 URL 路由。默认情况下，它会在 URL 中包含 `/post` 的页面上显示。

### 配置协作聊天

你可以告诉协作聊天只在特定区域内的 HTML 周围添加协作功能，例如，假设你在上面添加了页脚代码，然后在帖子/页面内容中添加此 div 来启用协作聊天：

[inline-code-attrs-start title = '具有指定内容的协作聊天'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

然后 `<div>` 内的段落元素将启用协作聊天，页面上的其他部分则不会。如果你在 `<div>` 中不放任何内容，那么它将对整个帖子正文启用协作聊天。