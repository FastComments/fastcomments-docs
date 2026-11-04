### Kullanım Senaryoları

Image Chat, ekiplerin mockup'larda veya ekran görüntülerinde belirli öğeleri tartışması gereken tasarım geri bildirimleri için mükemmel çalışır. Ürün inceleme siteleri, müşterilerin ürün fotoğraflarında görünen belirli özellikleri tartışmasını sağlayabilir. Eğitim platformları, diyagramları, haritaları veya bilimsel görselleri tartışmak için kullanabilir. Fotoğraf galerileri, konuma özgü yorumlarla etkileşimli hale gelebilir. Emlak siteleri, ziyaretçilerin mülk fotoğraflarında görünen belirli özellikleri tartışmasını sağlayabilir.

### Hızlı Başlangıç

Image Chat ile başlamak basittir. FastComments Image Chat betiğine, bir görsel içeren bir image öğesine veya konteynere ve Tenant ID'nizi içeren bir yapılandırma nesnesine ihtiyacınız var.

### Kurulum

Image Chat betiğini sayfanıza ekleyin:

[inline-code-attrs-start title = 'Image Chat Betiğini Yükleme'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Temel Uygulama

İşte minimal bir örnek:

[inline-code-attrs-start title = 'Temel Image Chat Uygulaması'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Image Gallery with Image Chat</title>
</head>
<body>
    <!-- Your image -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Load the Image Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Initialize Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Henüz değilse `'demo'` değerini gerçek FastComments Tenant ID'nizle değiştirin; bunu [FastComments panonuzda](https://fastcomments.com/auth/my-account) bulabilirsiniz.

### Nasıl Çalışır

Başlatıldıktan sonra, kullanıcılar görselin herhangi bir yerine tıklayabilir. Bir tıklama gerçekleştiğinde, o konumda görsel bir kare işaretçi belirir ve bir sohbet penceresi açılır. Diğer kullanıcılar tüm işaretçileri görebilir ve bu tartışmaları görüntülemek veya katılmak için onlara tıklayabilir. Tüm tartışmalar tüm ziyaretçiler arasında gerçek zamanlı olarak senkronize edilir.

Widget, yüzde tabanlı konumlandırma kullanır, böylece işaretçiler görsel yeniden boyutlandırıldığında veya farklı ekran boyutlarında görüntülendiğinde bile doğru konumda kalır.

### Canlı Demo

Image Chat'i çalışır halde [canlı demo sayfamızda](https://fastcomments.com/product/image-chat) görebilirsiniz.

### Sonraki Adımlar

Artık temelleri çalışır hale getirdiğinize göre, Görünümü ve davranışı Yapılandırma Seçenekleri kılavuzunda özelleştirebilirsiniz. Yüzde tabanlı konumlandırmanın nasıl çalıştığını anlamak için Duyarlı Tasarım kılavuzuna göz atın. Stil ve karanlık mod desteği hakkında Özelleştirme kılavuzundan bilgi edinin. Gelişmiş entegrasyonlar için API Referansını inceleyin.

### Frontend Kütüphaneleri

Tüm FastComments frontend kütüphanelerinde (react, vue, angular vb.) Image Chat bulunur.