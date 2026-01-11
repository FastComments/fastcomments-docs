Bir Geliştiricinin `Administrators` olmasını istemeyebileceğiniz durumlar için, bir `Administrator` kullanıcı oluşturmayı düşünün
aşağıdaki izinlere sahip:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Bu izin seti bir geliştiriciye FastComments'ı kurmak için ihtiyaç duyduğu her şeyi sağlayacak ve
sistemin çalıştığını doğrulamak için gereken görünürlüğü de sağlayacaktır.

The reasoning for these permissions is as follows:

1. **Analytics Admin**: Bu, FastComments kullanımını hata ayıklamak için kullanılabilir.
2. **Customizations Admin**: Bu, yorum bileşenine özel stil uygulamak için gerekecektir.
3. **Data Management Admin**: Bu, içe/dışa aktarımları yönetmek ve webhook'ları kurmak için gerekecektir.
4. **Comment Moderation Admin**: Bu, en azından kurulum sırasında yorum verilerini görmek için gerekecektir.
5. **API/SSO Admin**: Bu, onların API anahtarlarını doğrudan platformumuzdan almasını sağlayacaktır. Biz bunu
daha güvenli buluyoruz; bir `Administrator`'ın bunu onlar için kopyalayıp API Secret'ı e-posta ile göndermesinden
   bu çok güvenli olmayabilir.