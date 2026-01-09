### 深色模式 支持

### 动态深色模式

如果您站点的深色模式是通过向 body 元素添加 `.dark` 类来控制的，Collab Chat UI 会自动响应此类，而无需重新初始化。该小部件的样式设计为响应此类的存在。

[inline-code-attrs-start title = '深色模式 CSS 示例'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* 您的深色模式 CSS */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### 使用 CSS 自定义样式

您可以使用 CSS 自定义高亮、聊天窗口和其他元素的外观。该小部件会添加特定的类，您可以在样式表中针对这些类进行样式设置。

文本高亮使用 FastComments 的评论气泡样式系统，因此您对标准评论小部件所做的任何自定义也会影响 Collab Chat。

### 顶部栏 自定义

顶部栏显示在线用户数量和讨论数量。您可以通过提供一个自定义元素作为 `topBarTarget` 来自定义其位置：

[inline-code-attrs-start title = '自定义顶部栏位置'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

或者通过将其设置为 `null` 完全禁用：

[inline-code-attrs-start title = '禁用顶部栏'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### 移动端行为

在宽度小于 768px 的屏幕上，Collab Chat 会自动切换到针对移动设备优化的布局。聊天窗口会以全屏方式出现，而不是浮动在文本旁，并且会移除选择延迟以实现更即时的交互。

此行为为内置功能，无需任何配置。该小部件会自动检测屏幕大小并相应调整。

### 聊天窗口外观

桌面端聊天窗口宽度为 410px，并带有指向高亮文本的 16px 箭头。窗口会根据可用视口空间自动定位，使用像 `to-right`、`to-left`、`to-top` 和 `to-bottom` 这样的定位类。

您可以添加自定义 CSS 来调整这些窗口的颜色、字体、间距或其他视觉属性。聊天窗口使用与标准 FastComments 小部件相同的组件结构，因此它们会继承您对全局所做的任何自定义。

### 本地化

Collab Chat 支持与标准 FastComments 小部件相同的本地化选项。将 `locale` 选项设置为不同的语言以显示 UI 文本：

[inline-code-attrs-start title = '设置语言环境'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙语
});
[inline-code-end]

FastComments 支持数十种语言。locale 设置会影响所有 UI 文本，包括提示、按钮和占位符文本。

### 继承的自定义选项

由于 Collab Chat 扩展了标准的评论小部件，它继承了来自基础小部件的所有自定义选项。这包括自定义 CSS 类、自定义翻译、头像自定义、日期格式化等更多选项。

有关可用自定义选项的完整列表，请参阅 FastComments 的主要自定义文档。

### 使用自定义字体

如果您站点使用自定义字体，Collab Chat UI 会从页面的 CSS 中继承这些字体。如果您希望浮动聊天窗口使用相同的字体，您可能需要创建一个小部件自定义规则，并在该规则的自定义 CSS 中使用 `@import` 导入任何字体。