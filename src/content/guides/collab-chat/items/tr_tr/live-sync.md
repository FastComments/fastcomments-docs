### Gerçek Zamanlı Güncellemeler

Collab Chat, tüm bağlı kullanıcılar arasında tüm konuşmaları gerçek zamanlı olarak senkronize etmek için WebSocket bağlantılarını kullanır. Birisi yeni bir açıklama oluşturduğunda, bir yorum eklediğinde veya bir tartışmayı sildiğinde, aynı sayfayı görüntüleyen diğer tüm kullanıcılar sayfayı yenilemeden güncellemeyi hemen görür.

### WebSocket Senkronizasyonu Nasıl Çalışır

Collab Chat başlatıldığında, bileşen FastComments sunucularına bir WebSocket bağlantısı kurar. Bu bağlantı, kullanıcının oturumu süresince açık kalır ve geçerli sayfayla ilgili güncellemeleri dinler.

WebSocket sistemi Collab Chat için üç tür yayın mesajı kullanır. Sayfada birisi yeni bir açıklama oluşturduğunda `new-text-chat` olayı tetiklenir. Birisi mevcut bir konuşmayı güncellediğinde `updated-text-chat` olayı tetiklenir. Birisi bir açıklamayı sildiğinde `deleted-text-chat` olayı tetiklenir.

### Yayın ID Sistemi

Kullanıcıların kendi eylemlerinin kendilerine geri yayınlanmasıyla oluşan yankı etkilerini önlemek için her güncelleme benzersiz bir `broadcastId` içerir. Bir kullanıcı bir açıklama oluşturduğunda veya güncellediğinde, istemcisi bu işlem için bir UUID oluşturur. WebSocket güncellemeyi tüm istemcilere geri yayınladığında, kaynağı oluşturan istemci kendi `broadcastId` ile eşleştiği için güncellemeyi yoksayar.

Bu, kullanıcıların değişikliklerini sunucu üzerinden gidiş-dönüş beklemeden UI'da anında görmelerini sağlarken, diğer tüm kullanıcıların da güncellemeyi almasını garanti eder.

### Canlı Kullanıcı Sayısı

Üst çubuk, şu anda sayfayı görüntüleyen kullanıcı sayısını gösterir. Bu sayı, kullanıcılar katılıp ayrıldıkça gerçek zamanlı olarak güncellenir. Kullanıcı sayısı aynı WebSocket bağlantısı üzerinden sağlanır ve bağlantı ve bağlantı kesme olaylarına göre otomatik olarak artıp azalır.

### Bağlantı Dayanıklılığı

Ağ sorunları veya sunucu bakımı nedeniyle WebSocket bağlantısı koparsa, bileşen otomatik olarak yeniden bağlanmaya çalışır. Yeniden bağlanma süresince kullanıcılar mevcut açıklamalarla etkileşime devam edebilir, ancak bağlantı yeniden kurulana kadar diğer kullanıcıların gerçek zamanlı güncellemelerini göremeyeceklerdir.

Yeniden bağlanıldığında, bileşen kaçırılmış güncelleme olmadığından emin olmak için yeniden senkronize olur. Bu, kullanıcı müdahalesi gerektirmeden şeffaf şekilde gerçekleşir.

### Bant Genişliği Hususları

WebSocket mesajları hafiftir ve durumu senkronize etmek için gereken temel bilgileri içerir. Yeni bir açıklama oluşturmak genellikle 1KB'den az bant genişliği kullanır. Sistem ayrıca yüksek etkinlik dönemlerinde mesaj sıklığını azaltmak için akıllı paketleme (batching) içerir.

FastComments kontrol panelinizdeki kullanım ölçümleri, gerçek zamanlı senkronizasyon etkinliğini siteleriniz genelinde izlemeniz için `pubSubMessageCount` ve `pubSubBandwidth` öğelerini izler.

### Sekmeler Arası Senkronizasyon

Bir kullanıcı aynı sayfayı birden fazla tarayıcı sekmesinde açtıysa, bir sekmedeki güncellemeler diğer sekmelerde anında görünür. Bu, aynı WebSocket senkronizasyon mekanizmasıyla çalışır ve ek bir yapılandırma gerektirmez.

### Güvenlik

WebSocket mesajları güvenli bağlantılar (WSS) üzerinden iletilir ve kullanıcıların yalnızca yetkili oldukları konuşmalar için güncellemeler almasını sağlamak üzere kiracı (tenant) doğrulaması içerir. Sunucu, yetkisiz erişimi veya manipülasyonu önlemek için yayınlamadan önce tüm işlemleri doğrular.