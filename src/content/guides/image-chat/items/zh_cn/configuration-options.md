### 概述

FastComments Image Chat 扩展了标准的 FastComments 评论组件，因此它继承了基础组件的所有配置选项，同时添加了一些专门用于图片标注的选项。

### 必需的配置

#### tenantId

需要提供您的 FastComments 租户 ID。您可以在 [FastComments 控制台](https://fastcomments.com/auth/my-account/api-secret) 中找到它。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat 专属选项

#### urlId

默认情况下，Image Chat 会根据页面 URL、图片来源以及 X/Y 坐标为每个对话生成一个唯一标识符。您可以使用自定义的 `urlId` 来覆盖它。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

当您的 URL 结构可能发生变化但您希望保留相同的对话时，或者当您希望在多个页面之间共享标注时，这会非常有用。

#### chatSquarePercentage

控制可点击的聊天标记的大小，以图片宽度的百分比表示。默认值为 5%，即每个标记占图片宽度的 5%。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // 更大、更醒目的标记
});
```

较小的值会创建更不显眼的标记，更适合细节丰富的图片。较大的值使标记在内容繁杂的图片上更容易看到和点击，也更适合移动设备上的用户。

#### hasDarkBackground

当您的页面具有深色背景时，启用深色模式样式。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

一个回调函数，每当评论数量发生变化时触发。这对于更新徽章或页面标题等 UI 元素非常有用。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### 继承的配置选项

由于 Image Chat 扩展了标准评论组件，您可以使用基础 FastComments 组件的任何配置选项。以下是一些常用的选项：

#### locale

设置组件 UI 的语言。FastComments 支持数十种语言。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙语
});
```

#### readonly

将所有对话设为只读。用户可以查看现有的标记和讨论，但不能创建新的标记或回复。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso 和 simpleSSO

使用单点登录与您的身份验证系统集成。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO 配置
    }
});
```

有关身份验证选项的完整详细信息，请参阅 SSO 文档。

#### maxReplyDepth

控制回复可以嵌套的层级深度。默认情况下，Image Chat 将其设置为 0，这意味着所有评论都是扁平的（没有嵌套回复）。如果您想要线程化的对话，可以更改此设置。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 允许 3 层嵌套
});
```

### 内部配置

这些选项由 Image Chat 自动设置，不应被覆盖：

对于 Image Chat，`productId` 会自动设置为 `2`。`floating-chat` 扩展会自动加载以提供聊天窗口功能。组件会自动检测移动设备（宽度小于 768px 的屏幕），并相应地使用全屏聊天窗口调整 UI。

### 目标元素的灵活性

`FastCommentsImageChat` 的第一个参数可以是直接的 `<img>` 元素，也可以是包含图片的容器元素：

```javascript
// 直接的图片元素
FastCommentsImageChat(document.getElementById('my-image'), config);

// 包含图片的容器
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

如果您传入的是容器元素，组件会自动找到其中的图片。

### 完整示例

以下是一个同时展示多个配置选项的示例：

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // 您的 SSO 配置
    },
    maxReplyDepth: 1
});
```

有关从基础组件继承的所有可用配置选项的完整列表，请参阅 FastComments 主配置文档。