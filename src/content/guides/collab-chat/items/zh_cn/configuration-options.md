### 概览

FastComments Collab Chat 扩展了标准的 FastComments 评论小部件，因此继承了基础小部件的所有配置选项，同时增加了一些特定于文本注释的选项。

### 必需的配置

#### tenantId

需要提供您的 FastComments Tenant ID。您可以在您的 [FastComments 仪表盘](https://fastcomments.com/auth/my-account/api-secret) 中找到它。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat 特定选项

#### urlId

默认情况下，Collab Chat 会根据页面 URL、元素的 DOM 路径和选定的文本范围为每个会话生成唯一标识符。您可以使用自定义的 `urlId` 覆盖此行为。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

当您的 URL 结构可能发生变化但希望保留相同的会话，或希望在多个页面之间共享注释时，这很有用。

#### topBarTarget

控制显示顶部栏（显示用户计数和讨论计数）。设置为 `null` 可完全禁用顶部栏，或者提供一个 DOM 元素以在特定位置渲染它。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 禁用顶部栏
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// 在自定义位置渲染顶部栏
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

当页面具有深色背景时启用暗色模式样式。此检测是自动的，但有时需要手动覆盖。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

每当评论计数更改时触发的回调函数。可用于更新徽章或页面标题等 UI 元素。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### 继承的配置选项

由于 Collab Chat 扩展了标准评论小部件，您可以使用基础 FastComments 小部件的任何配置选项。以下是一些常用选项：

#### locale

设置小部件 UI 的语言。FastComments 支持数十种语言。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙语
});
[inline-code-end]

#### readonly

将所有会话设为只读。用户可以查看现有注释，但无法创建新注释或回复。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

使用单点登录与您的身份验证系统集成。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO 配置
    }
});
[inline-code-end]

有关身份验证选项的完整详情，请参阅 SSO 文档。

#### maxReplyDepth

控制回复的嵌套层级深度。默认情况下，Collab Chat 将此设置为 0，表示所有评论为平级（没有嵌套回复）。如果您想要线程化的会话，可以更改此设置。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 允许 3 级嵌套
});
[inline-code-end]

### 内部配置

以下选项由 Collab Chat 自动设置，不应被覆盖：

对于 Collab Chat，`productId` 会自动设置为 `3`。`floating-chat` 扩展会自动加载以提供聊天窗口功能。该小部件会自动检测移动设备（屏幕宽度低于 768px）并相应地调整 UI。

### 完整示例

下面是一个展示多个配置选项的示例：

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // 您的 SSO 配置
    },
    maxReplyDepth: 1
});
[inline-code-end]

有关从基础小部件继承的所有可用配置选项的完整列表，请参阅主 FastComments 配置文档。