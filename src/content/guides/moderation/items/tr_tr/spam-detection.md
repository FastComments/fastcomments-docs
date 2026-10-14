Varsayılan olarak, FastComments eğitilebilir bir spam algılama ile birlikte gelir.

Yorumları denetlerken yorumları **Spam** olarak işaretledikçe veya otomatik olarak **Spam** olarak bulunan yorumları **Not Spam** olarak işaretledikçe, spam algılama sistemi bu eylemlerden öğrenerek hangi yorumların spam olmasını istediğinizi daha doğru belirleyecektir.

**Spam** olarak işaretlenen yorumlar otomatik olarak onaylanmaz; bu yüzden açıkça **Not Spam** olarak işaretlenene kadar görünmezler.

Spam Algılama, Yorum Moderasyon Ayarları sayfasından devre dışı bırakılabilir.

### Farklı Spam Algılayıcılar

FastComments, spam tespit etmek için üç yöntemi destekler:

1. Sürekli eğitilen ve tüm FastComments.com kiracıları arasında paylaşılan geleneksel bir Naïve-Bayes sınıflandırıcısı.
2. Sürekli eğitilen ve kiracınıza **izole** edilmiş geleneksel bir Naïve-Bayes sınıflandırıcısı.
3. Bir LLM'in (büyük dil modeli) kullanılması.

Herkes paylaşılan ve izole Naïve-Bayes sınıflandırıcılarına erişebilir.

LLM seçeneği, Flex billing üzerindeyseniz Yorum Moderasyon Ayarları sayfasında seçilebilir; çünkü kullanılan token'lara göre faturalandırılır.

### Güven Faktörü

FastComments, belirli bir site için kullanıcının ne kadar güvenilir olduğuna bağlı olarak spam filtresini ayarlar.

Örneğin, yöneticiler kullanıcının birçok yorumunu sabitlediyse, muhtemelen çok güvenilir bir kullanıcıdır. Veya kullanıcı siteye uzun süredir üye olup çok sayıda yoruma sahipse, güven faktörü de yüksek olabilir.

### SSO

SSO kullanıcıları tarafından gönderilen yorumlar spam olarak değerlendirilebilir ve buna göre kontrol edilir. İstisna, SSO kullanıcısının e‑postası aşağıdaki izinlerden birine veya birkaçına sahip bir tenant kullanıcısıyla aynıysa geçerlidir:

- Account Owner
- Super Admin
- Comment Moderator Admin

Bu izinlere sahip SSO kullanıcılarının yorumları spam için kontrol edilmeyecektir.

### Tekrarlanan Mesajlar

FastComments tekrarlanan mesajları tespit eder ve engeller. Ayrıca spamı önlemeye yardımcı olmak için çok benzer olan tekrarlanan mesajları da tespit eder. Bu, platformumuzun suistimal amacıyla kullanılmasını önlediği için devre dışı bırakılamaz. Yüksek bir güven faktörünüz varsa, bu durum tekrarlanan mesajların engellenmesinde dikkate alınır.
