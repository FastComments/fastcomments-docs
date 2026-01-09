### 概述

FastComments Image Chat 扩展了标准的 FastComments 评论小部件，因此继承了基础小部件的所有配置选项，同时增加了一些针对图像注释的特定选项。

### 必需配置

#### tenantId

需要提供您的 FastComments Tenant ID。您可以在您的 [FastComments 仪表板](https://fastcomments.com/auth/my-account/api-secret) 中找到此项。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat 特定选项

#### urlId

默认情况下，Image Chat 会根据页面 URL、图片来源和 X/Y 坐标为每个会话生成一个唯一标识符。您可以使用自定义的 `urlId` 覆盖此值。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

当您的 URL 结构可能会发生变化但您仍希望保留相同的会话，或当您想在多个页面之间共享注释时，这非常有用。

#### chatSquarePercentage

控制可点击聊天标记的大小，以图片宽度的百分比表示。默认值为 5%，意味着每个标记占图片宽度的 5%。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // 更大、更明显的标记
});
```

较小的值会创建不那么显眼的标记，更适合用于细节丰富的图像。较大的值在繁忙的图片或移动设备上更容易查看和点击。

#### hasDarkBackground

当页面具有深色背景时启用深色模式样式。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

当评论数量发生变化时触发的回调函数。可用于更新徽章或页面标题等 UI 元素。

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

由于 Image Chat 扩展了标准评论小部件，因此您可以使用基础 FastComments 小部件的任何配置选项。以下是一些常用选项：

#### locale

设置小部件 UI 的语言。FastComments 支持数十种语言。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙语
});
```

#### readonly

使所有会话只读。用户可以查看现有标记和讨论，但无法创建新标记或回复。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

使用单点登录将其与您的身份验证系统集成。

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

控制回复可以嵌套的层数。默认情况下，Image Chat 将其设置为 0，表示所有评论都是平铺的（没有嵌套回复）。如果您想要线程式对话，可以更改此设置。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 允许 3 层嵌套
});
```

### 内部配置

这些选项由 Image Chat 自动设置，不应被覆盖：

对于 Image Chat，`productId` 会自动设置为 `2`。`floating-chat` 扩展会自动加载以提供聊天窗口功能。该小部件会自动检测移动设备（屏幕宽度小于 768px），并相应地使用全屏聊天窗口调整 UI。

### 目标元素灵活性

传递给 `FastCommentsImageChat` 的第一个参数可以是直接的 `<img>` 元素，也可以是包含图片的容器元素：

```javascript
// 直接的图像元素
FastCommentsImageChat(document.getElementById('my-image'), config);

// 包含图片的容器
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

如果您传入的是容器元素，小部件会自动查找其中的图片。

### 完整示例

下面是一个同时展示多个配置选项的示例：

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

有关从基础小部件继承的所有可用配置选项的完整列表，请参阅主要的 FastComments 配置文档。

---