### Hızlı Başlangıç

Collab Chat ile başlamak basittir. Sayfanıza FastComments Collab Chat betiğini, açıklama yapmak istediğiniz metni içeren bir HTML öğesini ve Tenant ID'niz ile bir yapılandırma nesnesini eklemeniz yeterlidir.

### Kurulum

Collab Chat betiğini sayfanıza ekleyin:

[inline-code-attrs-start title = 'Collab Chat Betiğini Yükleme'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Temel Uygulama

Here's a minimal example:

[inline-code-attrs-start title = 'Temel Collab Chat Uygulaması'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- İçerik kapsayıcınız -->
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

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### Nasıl Çalışır

Başlatıldıktan sonra kullanıcılar hedef öğe içindeki herhangi bir metni seçebilir. Kısa bir gecikmeden sonra (masaüstünde 3.5 saniye), bir istem görünür ve tartışma başlatmalarına izin verir. Bir tartışma oluşturulduğunda, metin üzerinde görsel bir vurgu görünür. Diğer kullanıcılar vurgunun üzerine gelerek veya tıklayarak tartışmayı görüntüleyebilir ve katılabilir. Tüm tartışmalar tüm ziyaretçiler arasında gerçek zamanlı olarak senkronize edilir.

### Canlı Demo

Collab Chat'i aksiyonda [canlı demo sayfamızda](https://fastcomments.com/product/collab-chat) görebilirsiniz.

### Sonraki Adımlar

Artık temel kurulum çalıştığına göre, görünüm ve davranışı Yapılandırma Seçenekleri kılavuzunda özelleştirebilirsiniz. Metin Seçimi Davranışı kılavuzuna bakarak metin seçiminin nasıl çalıştığını anlayın. Stil ve koyu mod desteği hakkında bilgi için Özelleştirme kılavuzuna göz atın. Gelişmiş entegrasyonlar için API Referansını inceleyin.

### Önyüz Kütüphaneleri

Tüm FastComments önyüz kütüphanelerinde (react, vue, angular, vb.) Collab Chat bulunur.