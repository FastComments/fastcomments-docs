### Gerçek Zamanlı Güncellemeler

Image Chat, tüm konuşmaları bağlı tüm kullanıcılar arasında gerçek zamanlı olarak senkronize etmek için WebSocket bağlantıları kullanır. Birisi yeni bir işaretleyici oluşturduğunda, yorum eklediğinde veya bir tartışmayı sildiğinde, aynı görüntüyü görüntüleyen diğer tüm kullanıcılar sayfayı yenilemeden güncellemeyi anında görür.

### WebSocket Senkronizasyonu Nasıl Çalışır

Image Chat'i başlattığınızda, widget FastComments sunucularına bir WebSocket bağlantısı kurar. Bu bağlantı, kullanıcının oturumu boyunca açık kalır ve mevcut görüntüyle ilgili güncellemeleri dinler.

WebSocket sistemi, Image Chat için üç tür yayın mesajı kullanır. `new-image-chat` olayı, birisi görüntü üzerinde yeni bir işaretleyici oluşturduğunda tetiklenir. `image-chat-updated` olayı, birisi mevcut bir konuşmayı güncellediğinde tetiklenir. `deleted-image-chat` olayı, birisi bir işaretleyiciyi sildiğinde tetiklenir.

### Yayın Kimliği Sistemi

Kullanıcıların kendi eylemlerinin kendilerine geri yayınlandığı yankı etkilerini önlemek için her güncelleme benzersiz bir `broadcastId` içerir. Bir kullanıcı bir işaretleyici oluşturduğunda veya güncellediğinde, istemcisi bu işlem için bir UUID oluşturur. WebSocket güncellemeyi tüm istemcilere geri yayınladığında, kaynak istemci, güncelleme kendi `broadcastId` değeriyle eşleştiği için onu yok sayar.

Bu, kullanıcıların değişikliklerini sunucu üzerinden gidiş-dönüşü beklemek zorunda kalmadan UI'da anında görmesini sağlarken, diğer tüm kullanıcıların da güncellemeyi almasını garanti eder.

### Bağlantı Dayanıklılığı

WebSocket bağlantısı ağ sorunları veya sunucu bakımı nedeniyle kesilirse, widget otomatik olarak yeniden bağlanmayı dener. Yeniden bağlanma süresi boyunca kullanıcılar mevcut işaretleyicilerle etkileşime devam edebilir, ancak bağlantı yeniden kurulana kadar diğer kullanıcılardan gelen gerçek zamanlı güncellemeleri göremezler.

Yeniden bağlandıktan sonra, widget hiçbir güncellemenin kaçırılmadığından emin olmak için yeniden senkronize olur. Bu, kullanıcı müdahalesi gerektirmeden şeffaf bir şekilde gerçekleşir.

### Bant Genişliği Hususları

WebSocket mesajları hafiftir ve durumu senkronize etmek için gereken yalnızca temel bilgileri içerir. Yeni bir işaretleyici oluşturmak genellikle 1KB'den daha az bant genişliği kullanır. Sistem ayrıca, yüksek etkinlik dönemlerinde mesaj sıklığını azaltmak için akıllı toplu işlem (batching) içerir.

FastComments panelindeki kullanım metrikleriniz `pubSubMessageCount` ve `pubSubBandwidth` değerlerini takip eder, böylece siteleriniz genelindeki gerçek zamanlı senkronizasyon etkinliğini izleyebilirsiniz.

### Sekmeler Arası Senkronizasyon

Bir kullanıcının aynı sayfayı birden fazla tarayıcı sekmesinde açık tutması durumunda, bir sekmedeki güncellemeler diğer sekmelerde anında görünür. Bu, aynı WebSocket senkronizasyon mekanizmasıyla çalışır ve ek yapılandırma gerektirmez.

Kullanıcılar sitenizi aynı anda birden fazla cihazda açık tutabilir ve tümü senkronize kalır. Masaüstü bilgisayarda oluşturulan bir işaretleyici, her iki cihaz da aynı görüntüyü görüntülüyorsa kullanıcının tabletinde anında görünür.

### Güvenlik

WebSocket mesajları güvenli bağlantılar (WSS) üzerinden iletilir ve kullanıcıların yalnızca görmeye yetkili oldukları konuşmaların güncellemelerini almasını sağlamak için kiracı (tenant) doğrulaması içerir. Sunucu, yetkisiz erişimi veya manipülasyonu önlemek için tüm işlemleri yayınlamadan önce doğrular.

### Çevrimdışı Davranış

Kullanıcılar tamamen çevrimdışıyken mevcut işaretleyicileri görüntülemeye devam edebilir, ancak yenilerini oluşturamaz veya başkalarından gelen güncellemeleri göremezler. Widget çevrimdışı durumu algılar ve uygun bir mesaj görüntüler.

Bir kullanıcı çevrimdışıyken bir işaretleyici oluşturmaya çalışır ve ardından tekrar çevrimiçi olursa, işlem veri tutarlılığını sağlamak için kuyruğa alınmak yerine başarısız olur. Kullanıcılar bağlantıları geri yüklendikten sonra işlemi yeniden denemelidir.

### Performans Etkisi

WebSocket bağlantısının performans üzerindeki etkisi minimumdur. Güncelleme olmadığında bağlantı boşta kalır ve yalnızca etkinlik gerçekleştiğinde mesajları işler. Orta düzeyde işaretleyici etkinliğine sahip tipik bir görüntüde, WebSocket, görüntünün kendisini oluşturmaktan daha az CPU kullanır.

Aynı anda yüzlerce kullanıcıya ve yüksek işaretleyici oluşturma etkinliğine sahip sayfalarda, sistem bireysel istemci bağlantılarını etkilemeden performansı korumak için yatay olarak ölçeklenir.

### İşbirliğine Dayalı Kullanım Senaryoları

Gerçek zamanlı senkronizasyon, Image Chat'i işbirliğine dayalı iş akışları için özellikle güçlü kılar. Tasarım ekipleri, herkes işaretleyici yerleşimlerini gerçek zamanlı görerek mockup'ları birlikte inceleyebilir. Müşteri destek ekipleri, sorunları belirlemek için ekran görüntülerini işbirliği içinde işaretleyebilir. Eğitim grupları, tüm katılımcıların birbirlerinin işaretleyicilerini oluşturuldukları anda gördüğü diyagramları tartışabilir.

Anında geri bildirim, kullanıcıların güncellemeleri görmek için yenileme yapması gereken geleneksel yorum sistemlerine kıyasla daha ilgi çekici ve üretken bir işbirliği deneyimi yaratır.