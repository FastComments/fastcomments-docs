### 概述

FastComments Collab Chat 扩展了标准的 FastComments 评论组件，因此它继承了基础组件的所有配置选项，同时增加了一些针对文本标注的特定选项。

### 必需的配置

#### tenantId

必须提供您的 FastComments 租户 ID（Tenant ID）。您可以在 [FastComments 控制台](https://fastcomments.com/auth/my-account/api-secret) 中找到它。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat 特有选项

#### urlId

默认情况下，Collab Chat 会根据页面 URL、元素的 DOM 路径以及选中的文本范围，为每个会话生成一个唯一标识符。您可以通过自定义的 `urlId` 来覆盖此行为。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

当您的 URL 结构可能发生变化但您希望保留相同的会话，或者您希望在多个页面之间共享标注时，这会非常有用。

#### topBarTarget

控制顶栏的显示，顶栏显示用户数量和讨论数量。设置为 `null` 可完全禁用顶栏，或者提供一个 DOM 元素以在特定位置渲染顶栏。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 禁用顶栏
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// 在自定义位置渲染顶栏
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

当您的页面具有深色背景时，启用深色模式样式。此检测是自动进行的，但您可能希望覆盖它。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

一个回调函数，每当评论数量发生变化时触发。这对于更新徽章或页面标题等 UI 元素非常有用。

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

由于 Collab Chat 扩展了标准评论组件，您可以使用基础 FastComments 组件的任何配置选项。以下是一些常用选项：

#### locale

设置组件 UI 的语言。FastComments 支持数十种语言。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙语
});
[inline-code-end]

#### readonly

将所有会话设为只读。用户可以查看现有的标注，但不能创建新标注或回复。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso 和 simpleSSO

使用单点登录（Single Sign-On）与您的身份验证系统集成。

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

控制回复可以嵌套多少层。默认情况下，Collab Chat 将此值设置为 0，这意味着所有评论都是扁平的（没有嵌套回复）。如果您想要线程化的会话，可以更改此值。

[inline-code-attrs-start title = "配置示例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 允许 3 层嵌套
});
[inline-code-end]

### 内部配置

这些选项由 Collab Chat 自动设置，不应被覆盖：

对于 Collab Chat，`productId` 会自动设置为 `3`。`floating-chat` 扩展会自动加载以提供聊天窗口功能。组件会自动检测移动设备（宽度小于 768px 的屏幕）并相应地调整 UI。

### 完整示例

以下是一个同时展示多个配置选项的示例：

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

有关从基础组件继承的所有可用配置选项的完整列表，请参阅 FastComments 主配置文档。