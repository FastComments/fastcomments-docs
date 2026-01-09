### Temel Örnek

Image Chat'i kullanmanın en basit yolu tek bir görüntü elementini hedeflemektir. Bu örnek, bir görüntü üzerinde etkileşimli tartışmaları nasıl etkinleştireceğinizi gösterir:

[inline-code-attrs-start title = 'Temel Image Chat Örneği'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Konteyner Element ile Örnek

Ayrıca içinde bir görüntü olan bir konteyner elementi de verebilirsiniz:

[inline-code-attrs-start title = 'Konteyner İçeren Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### Özel URL ID ile Örnek

Varsayılan olarak Image Chat, konuşmaları tanımlamak için sayfa URL'sini görüntü kaynağı ve koordinatlarla birleştirir. Özel bir `urlId` sağlayabilirsiniz:

[inline-code-attrs-start title = 'Özel URL ID ile Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        urlId: 'product-v2-main-image'
    });
</script>
[inline-code-end]

URL yapınız değişse bile aynı konuşmaları korumak istiyorsanız ya da aynı tartışma noktalarını birden fazla sayfa arasında paylaşmak istiyorsanız bu yararlıdır.

### Karanlık Mod ile Örnek

Sitenizin arka planı koyuysa ve widget bunu otomatik olarak algılamıyorsa, karanlık mod desteğini manuel olarak etkinleştirebiliriz:

[inline-code-attrs-start title = 'Karanlık Mod ile Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    FastCommentsImageChat(document.getElementById('product-image'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]

### Özel Sohbet Karesi Boyutu ile Örnek

Görüntü üzerinde görünen tıklanabilir karelerin boyutunu ayarlayabilirsiniz. Boyut, görüntü genişliğinin yüzdesi olarak belirtilir:

[inline-code-attrs-start title = 'Özel Kare Boyutu ile Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            chatSquarePercentage: 2, // Daha küçük kareler (varsayılan 5)
        });
    </script>
</body>
</html>
[inline-code-end]

### Yorum Sayısı Geri Çağrısı ile Örnek

Yorumlar eklendiğinde veya güncellendiğinde `commentCountUpdated` geri çağrısını kullanarak takip edebilirsiniz:

[inline-code-attrs-start title = 'Yorum Sayısı Geri Çağrısı ile Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

### Birden Çok Görüntü ile Örnek

Image Chat'i birden çok görüntüde başlatabilirsiniz. Her görüntünün kendi bağımsız tartışma noktaları olacaktır:

[inline-code-attrs-start title = 'Birden Çok Görüntüde Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<img id="image-1" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 1" />
<img id="image-2" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Photo 2" />

<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
<script>
    // İlk görüntüde başlat
    FastCommentsImageChat(document.getElementById('image-1'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-1'
    });

    // İkinci görüntüde başlat
    FastCommentsImageChat(document.getElementById('image-2'), {
        tenantId: 'demo',
        urlId: 'gallery-photo-2'
    });
</script>
[inline-code-end]

---