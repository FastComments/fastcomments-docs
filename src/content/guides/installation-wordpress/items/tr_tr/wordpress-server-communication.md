Eklentinin çalışması için, bir token WordPress veritabanınızda ve FastComments hesabınızda saklanır. Eklenti sunucularımıza istek yaptığında, bu token'ı sağlar
bu token.

FastComments hesabınıza yetki verilmiş tüm entegrasyonları [buradan](https://fastcomments.com/auth/my-account/manage-data/integrations) görüntüleyebilirsiniz.

Tüm iletişim HTTPS üzerinden gerçekleştirilir.

Tüm iletişim WordPress sunucunuzdan *giden* *FastComments.com'a* yöneliktir; WordPress kurulumunuza yapılan *geri* senkronizasyon da dahildir; bu senkronizasyon,
[polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) yoluyla WordPress kurulumunuzdaki bir [cron](https://developer.wordpress.org/plugins/cron/) yapılandırmasından uygulanır.