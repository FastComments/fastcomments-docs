### Gerekli Bileşenler

On-Prem için FastComments sadece bir uygulama sunucusu ve bir veritabanından oluşur. Uygulamanın diğer bileşenleri eklemeden tüm trafiği doğrudan sunabilmesi için dağıtımı basitleştirdik.

Uygulama sunucusu bir Docker imajı olarak sağlanır ve herhangi bir konteyner yönetim çözümüyle dağıtılabilir.

Veritabanı, MongoDB, kendi başınıza çalıştırılabilir veya AWS DocumentDB veya MongoDB Atlas gibi başka bir sağlayıcı tarafından barındırılabilir.

FastComments şu anda MongoDB 7 ile test edilmiştir, ancak dağıtımı kolaylaştırmak için DocumentDB ile uyumlu olmayı hedefliyoruz.

### Örnek Boyutları

FastComments'in oldukça iyi optimize edildiğini ve uygulamanın kendisi için düşük P99'ları korumak amacıyla büyük makinelere ihtiyaç duymadığını göreceksiniz.

Tüm toplu ve cron işleri toplam bellek kullanımını sınırlamak için streaming kullanır.

Aşağıdaki tablolar uygulama sunucusu ve veritabanı için boyutlandırmaya yardımcı olabilir.

### Uygulama Sunucusu Örnekleri


| Eşzamanlı Kullanıcılar | Toplam Küme CPU'ları | Toplam Küme Belleği |
|------------------------|----------------------|----------------------|
| 100                    | 1                    | 256mb                |
| 1K                     | 2                    | 512mb                |
| 10K                    | 8                    | 1gb                  |
| 100K                   | 32                   | 8gb                  |
| 1M                     | 64                   | 64gb                 |

Örneğin, saniyede yaklaşık 100 yorum dizisini sunan tek bir çekirdek genellikle 250mb RSS'den fazla kullanmaz.

### Veritabanı Sunucusu Örnekleri

Veritabanının boyutlandırılması, belirli bir zamanda eriştiğiniz veri miktarı olan çalışma seti boyutuna ve eşzamanlı isteklere bağlıdır.

FastComments Mongo'ya karşı oldukça naziktir; hot sorgular için index hints, streaming cursors kullanır ve aşağı akış sistemlerin aşırı yüklenmesini önlemek için çeşitli alanlarda concurrency limits içerir.

Aşağıdakiler veritabanı örneği boyutları için genel bir kılavuz niteliğindedir. **Bunun __her bir örnek için__ olduğunu, kümedeki toplam kaynaklar olmadığını unutmayın.**

| Eşzamanlı Kullanıcılar | Saklanan Yorumlar | Her Örnek İçin CPU'lar | Her Örnek İçin Bellek |
|------------------------|-------------------|------------------------|-----------------------|
| 100                    | 1k                | 1                      | 256mb                 |
| 1K                     | 5k                | 2                      | 512mb                 |
| 10K                    | 100k              | 8                      | 2gb                   |
| 100K                   | 500k              | 16                     | 8gb                   |
| 1M                     | 5M                | 32                     | 32gb                  |

Yukarıdaki tablolar muhafazakar tahminlerdir. Gerçek gereksinimlerin sayfa boyutları, yorum hacmi vb. gibi sizin belirli yapılandırmanıza bağlı olarak farklılık gösterebileceğini görebilirsiniz.