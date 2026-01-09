### 基本示例

使用 Image Chat 的最简单方法是针对单个图像元素。此示例演示如何在图像上启用交互式讨论：

[inline-code-attrs-start title = '基本图像聊天示例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Product Image with Chat</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

### 带容器元素的示例

您也可以传入一个内部包含图像的容器元素：

[inline-code-attrs-start title = '带容器的图像聊天'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<div id="image-container">
    <img src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="System Diagram" />
</div>

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('image-container'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

### 带自定义 URL ID 的示例

默认情况下，Image Chat 使用页面 URL 与图像源和坐标的组合来标识对话。您可以提供自定义的 `urlId`：

[inline-code-attrs-start title = '带自定义 URL ID 的图像聊天'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

如果您的 URL 结构发生变化但希望保持相同的讨论，或者希望在多个页面之间共享相同的讨论点，这会很有用。

### 带深色模式的示例

如果您的网站具有深色背景且小部件没有像应该那样自动检测到，我们可以手动启用深色模式支持：

[inline-code-attrs-start title = '带深色模式的图像聊天'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### 带自定义聊天方块大小的示例

您可以调整出现在图像上的可点击方块的大小。大小以图像宽度的百分比指定：

[inline-code-attrs-start title = '带自定义方块大小的图像聊天'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Image Chat with Custom Square Size</title>
</head>
<body>
    <img id="product-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Product Photo" />

    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
    <script>
        FastCommentsImageChat(document.getElementById('product-image'), {
            tenantId: 'demo',
            chatSquarePercentage: 2, // 更小的方块（默认是 5）
        });
    </script>
</body>
</html>
[inline-code-end]

### 带评论计数回调的示例

使用 `commentCountUpdated` 回调来跟踪何时添加或更新评论：

[inline-code-attrs-start title = '带评论计数回调的图像聊天'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        commentCountUpdated: function(count) {
            console.log('Total comments:', count);
            document.getElementById('comment-badge').textContent = count;
        }
    });
</script>
[inline-code-end]

### 多图像示例

您可以在多张图像上初始化 Image Chat。每张图像都会有自己独立的讨论点：

[inline-code-attrs-start title = '在多张图片上的图像聊天'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // 在第一张图像上初始化
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // 在第二张图像上初始化
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---