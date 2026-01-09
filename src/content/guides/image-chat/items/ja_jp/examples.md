### 基本的な例

Image Chat を使用する最も簡単な方法は、単一の画像要素をターゲットにすることです。この例は、画像上でインタラクティブなディスカッションを有効にする方法を示しています:

[inline-code-attrs-start title = '基本的な Image Chat の例'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### コンテナ要素の例

画像を内包するコンテナ要素を渡すこともできます:

[inline-code-attrs-start title = 'コンテナを使用した Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### カスタム URL ID を使用した例

デフォルトでは、Image Chat はページの URL と画像のソースおよび座標を組み合わせて会話を識別します。カスタムの `urlId` を指定できます:

[inline-code-attrs-start title = 'カスタム URL ID を使用した Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

URL 構造が変更されても同じ会話を維持したい場合や、複数のページで同じ議論ポイントを共有したい場合に便利です。

### ダークモードの例

サイトがダークな背景を持ち、ウィジェットが自動的に検出しない場合は、手動でダークモードを有効にできます:

[inline-code-attrs-start title = 'ダークモード対応の Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### チャットスクエアのカスタムサイズの例

画像上に表示されるクリック可能な四角（スクエア）のサイズを調整できます。サイズは画像幅に対するパーセンテージで指定します:

[inline-code-attrs-start title = 'カスタムスクエアサイズの Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // より小さい四角（デフォルトは 5）
        });
    </script>
</body>
</html>
[inline-code-end]

### コメント数コールバックの例

コメントが追加または更新されたときは `commentCountUpdated` コールバックで追跡できます:

[inline-code-attrs-start title = 'コメント数コールバックの Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### 複数画像での例

複数の画像に対して Image Chat を初期化できます。各画像は独立したディスカッションポイントを持ちます:

[inline-code-attrs-start title = '複数画像での Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // 最初の画像で初期化
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // 2番目の画像で初期化
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---