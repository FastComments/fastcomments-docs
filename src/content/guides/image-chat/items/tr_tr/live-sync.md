### Real-Time Updates

Image Chat, tüm bağlı kullanıcılar arasında tüm konuşmaları gerçek zamanlı senkronize etmek için WebSocket bağlantıları kullanır. Biri yeni bir işaretçi oluşturduğunda, yorum eklediğinde veya bir tartışmayı sildiğinde, aynı resmi görüntüleyen diğer tüm kullanıcılar sayfayı yenilemeye gerek kalmadan güncellemeyi hemen görür.

### How WebSocket Sync Works

Image Chat başlatıldığında, widget FastComments sunucularına bir WebSocket bağlantısı kurar. Bu bağlantı kullanıcının oturumu süresince açık kalır ve geçerli resimle ilgili güncellemeleri dinler.

WebSocket sistemi Image Chat için üç tür yayın mesajı kullanır. `new-image-chat` olayı birisi resme yeni bir işaretçi oluşturduğunda tetiklenir. `image-chat-updated` olayı birisi mevcut bir konuşmayı güncellediğinde tetiklenir. `deleted-image-chat` olayı birisi bir işaretçiyi sildiğinde tetiklenir.

### Broadcast ID System

Kullanıcıların kendi eylemlerinin kendilerine geri yayıldığını görerek yankı etkilerini önlemek için her güncelleme benzersiz bir `broadcastId` içerir. Bir kullanıcı bir işaretçi oluşturduğunda veya güncellediğinde, istemcisi o işlem için bir UUID üretir. WebSocket güncellemeyi tüm istemcilere yayınladığında, kaynağı oluşturan istemci kendi `broadcastId` ile eşleştiği için güncellemeyi yok sayar.

Bu, kullanıcıların sunucu üzerinden gidip gelmeyi beklemeden değişikliklerini arayüzde anında görmesini sağlarken, diğer tüm kullanıcıların da güncellemeyi almasını garanti eder.

### Connection Resilience

WebSocket bağlantısı ağ sorunları veya sunucu bakımı nedeniyle kesilirse, widget otomatik olarak yeniden bağlanmaya çalışır. Yeniden bağlanma süresince kullanıcılar mevcut işaretçilerle etkileşime devam edebilir, ancak bağlantı yeniden kurulana kadar diğer kullanıcıların gerçek zamanlı güncellemelerini göremezler.

Bir kez yeniden bağlanıldığında, widget herhangi bir güncellemenin kaçırılmadığından emin olmak için yeniden senkronize olur. Bu, kullanıcı müdahalesi gerektirmeden şeffaf bir şekilde gerçekleşir.

### Bandwidth Considerations

WebSocket mesajları hafiftir ve yalnızca durumu senkronize etmek için gerekli temel bilgileri içerir. Yeni bir işaretçi oluşturmak genellikle 1KB'den daha az bant genişliği kullanır. Sistem ayrıca yüksek etkinlik dönemlerinde mesaj sıklığını azaltmak için akıllı toplu işlemeyi içerir.

FastComments kontrol panelinizdeki kullanım metrikleriniz `pubSubMessageCount` ve `pubSubBandwidth` ile gerçek zamanlı senkronizasyon etkinliğini siteleriniz genelinde izlemenizi sağlar.

### Cross-Tab Synchronization

Kullanıcının aynı sayfayı birden fazla tarayıcı sekmesinde açık tutması durumunda, bir sekmedeki güncellemeler diğer sekmelerde hemen görünür. Bu, aynı WebSocket senkronizasyon mekanizmasıyla çalışır ve ek bir yapılandırma gerektirmez.

Kullanıcılar sitenizi aynı anda birden fazla cihazda açık tutabilir ve hepsi senkronize kalır. Masaüstünde oluşturulan bir işaretçi, her iki cihaz da aynı resmi görüntülüyorsa kullanıcının tabletinde anında görünür.

### Security

WebSocket mesajları güvenli bağlantılar (WSS) üzerinden aktarılır ve kullanıcıların yalnızca yetkili oldukları konuşmalar için güncellemeleri almasını sağlamak üzere kiracı doğrulaması içerir. Sunucu, yetkisiz erişimi veya manipülasyonu önlemek için yayınlamadan önce tüm işlemleri doğrular.

### Offline Behavior

Kullanıcılar tamamen çevrimdışıyken mevcut işaretçileri görüntüleyebilirler ancak yeni bir işaretçi oluşturamaz veya başkalarının güncellemelerini göremezler. Widget çevrimdışı durumu algılar ve uygun bir ileti gösterir.

Bir kullanıcı çevrimdışıyken bir işaretçi oluşturmaya çalışır ve ardından çevrimiçi hale gelirse, işlem kuyruklanmak yerine başarısız olur; bu, veri tutarlılığını sağlar. Kullanıcılar bağlantıları geri geldiğinde işlemi yeniden denemelidir.

### Performance Impact

WebSocket bağlantısının performans üzerinde minimal etkisi vardır. Güncelleme olmadığında bağlantı boşta kalır ve yalnızca etkinlik olduğunda mesajları işler. Orta düzey işaretçi etkinliğine sahip tipik bir resimde, WebSocket görüntünün kendisini render etmekten daha az CPU kullanır.

Yüzlerce eşzamanlı kullanıcı ve yüksek işaretçi oluşturma etkinliği olan sayfalarda, sistem bireysel istemci bağlantılarını etkilemeden performansı korumak için yatay olarak ölçeklenir.

### Collaborative Use Cases

Gerçek zamanlı senkronizasyon, Image Chat'i işbirlikçi iş akışları için özellikle güçlü kılar. Tasarım ekipleri, herkes işaretçi yerleşimlerini gerçek zamanlı görerek mockup'ları birlikte gözden geçirebilir. Müşteri destek ekipleri, sorunları belirlemek için ekran görüntülerini birlikte notlandırabilir. Eğitim grupları, oluşturuldukça herkesin birbirinin işaretçilerini gördüğü diyagramları tartışabilir.

Anında geri bildirim, kullanıcıların güncellemeleri görmek için yenileme yapması gereken geleneksel yorum sistemlerine kıyasla daha ilgi çekici ve üretken bir işbirliği deneyimi yaratır.