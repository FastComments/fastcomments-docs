### Hızlı Başlangıç

Collab Chat ile başlamak çok basittir. FastComments Collab Chat betiğine, açıklama eklemek istediğiniz metni içeren bir HTML öğesine ve Tenant ID'nizi içeren bir yapılandırma nesnesine ihtiyacınız var.

### Kurulum

Collab Chat betiğini sayfanıza ekleyin:

[inline-code-attrs-start title = 'Collab Chat Betiğini Yükleme'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Temel Uygulama

İşte minimal bir örnek:

[inline-code-attrs-start title = 'Temel Collab Chat Uygulaması'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- İçerik konteyneriniz -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Collab Chat betiğini yükleyin -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Collab Chat'i başlatın -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

`'demo'` değerini, henüz yapmadıysanız gerçek FastComments Tenant ID'nizle değiştirin; bunu [FastComments kontrol panelinizde](https://fastcomments.com/auth/my-account/api-secret) bulabilirsiniz.

### Nasıl Çalışır

Başlatıldıktan sonra, kullanıcılar hedef öğe içindeki herhangi bir metni seçebilir. Kısa bir gecikmenin ardından (masaüstünde 3,5 saniye), bir tartışma başlatmalarına olanak tanıyan bir istem görünür. Bir tartışma oluşturulduğunda, metin üzerinde görsel bir vurgu belirir. Diğer kullanıcılar, tartışmayı görüntülemek ve katılmak için vurgunun üzerine gelebilir veya tıklayabilir. Tüm tartışmalar tüm ziyaretçiler arasında gerçek zamanlı olarak senkronize edilir.

### Canlı Demo

Collab Chat'i çalışırken [canlı demo sayfamızda](https://fastcomments.com/product/collab-chat) görebilirsiniz.

### Sonraki Adımlar

Artık temelleri çalışır hale getirdiğinize göre, görünümü ve davranışı Yapılandırma Seçenekleri kılavuzunda özelleştirebilirsiniz. Metin seçiminin nasıl çalıştığını anlamak için Metin Seçim Davranışı kılavuzuna göz atın. Stil ve karanlık mod desteği hakkında bilgi edinmek için Özelleştirme kılavuzuna bakın. Gelişmiş entegrasyonlar için API Referansını inceleyin.

### Frontend Kütüphaneleri

Tüm FastComments frontend kütüphanelerinde (react, vue, angular vb.) Collab Chat bulunur.