### Kullanım Senaryoları

Image Chat, ekiplerin mockup veya ekran görüntülerindeki belirli öğeleri tartışması gerektiği tasarım geri bildirimleri için mükemmeldir. Ürün inceleme siteleri, müşterilerin ürün fotoğraflarında görünen belirli özellikleri tartışmasına izin verebilir. Eğitim platformları, diyagramlar, haritalar veya bilimsel görüntüler hakkında tartışmak için kullanabilir. Fotoğraf galerileri, konuma özgü yorumlarla etkileşimli hale gelebilir. Emlak siteleri, izleyicilerin mülk fotoğraflarında görünen belirli özellikleri tartışmasına olanak tanıyabilir.

### Hızlı Başlangıç

Image Chat ile başlamak basittir. FastComments Image Chat betiğine, bir img öğesine veya görüntü içeren bir kap konteynerine ve Tenant ID'niz içeren bir yapılandırma nesnesine ihtiyacınız vardır.

### Kurulum

Sayfanıza Image Chat betiğini ekleyin:

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
    <!-- Resminiz -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Image Chat betiğini yükleyin -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Image Chat'i Başlatın -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account).

### Nasıl Çalışır

Başlatıldıktan sonra kullanıcılar resmin her hangi bir yerine tıklayabilir. Bir tıklama olduğunda, o konumda görsel bir kare işaretçi belirir ve bir sohbet penceresi açılır. Diğer kullanıcılar tüm işaretçileri görebilir ve bu tartışmaları görüntülemek veya katılmak için onları tıklayabilir. Tüm tartışmalar tüm ziyaretçiler arasında gerçek zamanlı olarak senkronize edilir.

Widget, yüzde tabanlı konumlandırma kullanır, bu nedenle görüntü yeniden boyutlandırıldığında veya farklı ekran boyutlarında görüntülendiğinde işaretçiler doğru konumda kalır.

### Canlı Demo

Image Chat’i eylemde [canlı demo sayfamızda](https://fastcomments.com/product/image-chat) görebilirsiniz.

### Sonraki Adımlar

Artık temel yapı çalıştığına göre, Görünüm ve davranışı yapılandırma seçenekleri kılavuzunda özelleştirebilirsiniz. Yüzde tabanlı konumlandırmanın nasıl çalıştığını anlamak için Duyarlı Tasarım kılavuzuna göz atın. Stil ve karanlık mod desteği hakkında bilgi için Özelleştirme kılavuzunu inceleyin. İleri entegrasyonlar için API Referansını keşfedin.

### Önyüz Kütüphaneleri

Tüm FastComments önyüz kütüphanelerinde (react, vue, angular, vb.) Image Chat bulunmaktadır.