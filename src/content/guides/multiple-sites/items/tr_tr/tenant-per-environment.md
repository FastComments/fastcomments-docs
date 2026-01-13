FastComments ile test veya geliştirme ortamı başına bir alt tenant olması yaygındır. Her tenant kendi yapılandırmasına, verisine ve API anahtarlarına sahip olur. Yapılandırma, veri ve kullanıcılar tenant'lar arasında paylaşılamaz.
Her şey izole edilmiştir. Ancak, üst tenant'ın süper yöneticileri alt tenant'lardaki kullanıcıların yerine geçebilir.

İki yaklaşım vardır:

- Ana tenant üretim içindir ve alt tenant'lar test ortamları içindir.
- Ana tenant yalnızca faturalama içindir ve her alt-tenant prod, test vb. için ayrılır.

İlki genellikle kullanıcıların kavraması için daha kolaydır, ancak bu kuruluşunuza bağlı olabilir.

Tenant'lar [buradan](https://eu.fastcomments.com/auth/my-account/tenants) oluşturulabilir eğer pakete sahipseniz. Burası ayrıca süper yöneticilerin
kullanıcıların yerine geçeceği yerdir. Daha özel/otomatik kurulumlar için tenant'lar API üzerinden de oluşturulabilir.

Hangi yaklaşım seçilirse seçilsin, üretim verilerini görmek isteyen moderatörleri ve kullanıcıları "prod" tenant'ına eklemeniz gerekecektir. Örneğin, eğer seçenek B'yi
seçip üst tenant'ı faturalama için kullanmak ve "prod" için bir alt tenant oluşturmak istiyorsanız, tenant'ı eklemeli, yeni tenant'a geçmeli ve alt-tenant için
admin ve moderatör kullanıcılarınızı eklemelisiniz. 

Son olarak, açıklık getirmek için, üst tenant için seçenek B ile Yorumları Denetle sayfası boş olacaktır.