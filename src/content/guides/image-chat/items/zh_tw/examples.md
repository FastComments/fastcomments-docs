### 基本範例

使用 Image Chat 最簡單的方法是針對單一圖片元素。此範例示範如何在圖片上啟用互動討論：

[inline-code-attrs-start title = '基本 Image Chat 範例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### 使用容器元素的範例

你也可以傳入一個內含圖片的容器元素：

[inline-code-attrs-start title = '使用容器的 Image Chat 範例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### 使用自訂 URL ID 的範例

預設情況下，Image Chat 使用頁面 URL 結合圖片來源與座標來識別對話。你可以提供自訂的 `urlId`：

[inline-code-attrs-start title = '使用自訂 URL ID 的 Image Chat 範例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

如果你的 URL 結構改變但想保留相同的對話，或想在多個頁面間共用相同的討論點，這會很有用。

### 使用深色模式的範例

如果你的網站具有深色背景，而元件未像應該的自動偵測到，我們可以手動啟用深色模式支援：

[inline-code-attrs-start title = '使用深色模式的 Image Chat 範例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### 自訂聊天方塊大小的範例

你可以調整顯示在圖片上的可點擊方塊的大小。大小以圖片寬度的百分比來指定：

[inline-code-attrs-start title = '使用自訂方塊大小的 Image Chat 範例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // 較小的方塊 (預設為 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### 使用評論數量回調的範例

使用 `commentCountUpdated` 回調來追蹤評論新增或更新的情況：

[inline-code-attrs-start title = '使用評論計數回調的 Image Chat 範例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### 多張圖片範例

你可以在多張圖片上初始化 Image Chat。每張圖片都會有獨立的討論點：

[inline-code-attrs-start title = '在多張圖片上的 Image Chat 範例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // 在第一張圖片上初始化
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // 在第二張圖片上初始化
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---