Varsayılan olarak, FastComments eğitilebilir bir spam algılama ile birlikte gelir.

As you moderate comments, and mark them as **Spam**, or mark comments automatically found as **Spam** as **Not Spam**, the spam
detection system will learn from these actions to more accurately determine what you want to be spam.

Yorumları denetlerken yorumları **Spam** olarak işaretledikçe veya otomatik olarak **Spam** olarak bulunan yorumları **Not Spam** olarak işaretledikçe, spam algılama sistemi bu eylemlerden öğrenerek hangi yorumların spam olmasını istediğinizi daha doğru belirleyecektir.

Comments marked as **Spam** will not be automatically approved, so they will not show until explicitly marked as **Not Spam**.

**Spam** olarak işaretlenen yorumlar otomatik olarak onaylanmaz; bu yüzden açıkça **Not Spam** olarak işaretlenene kadar görünmezler.

Spam Detection can be disabled via the Comment Moderation Settings page.

Spam Algılama, Yorum Moderasyon Ayarları sayfasından devre dışı bırakılabilir.

### Different Spam Detectors

### Farklı Spam Algılayıcılar

FastComments supports three ways of detecting spam:

FastComments, spam tespit etmek için üç yöntemi destekler:

1. A traditional Naïve-Bayes classifier that is continuously trained, which is shared across all FastComments.com tenants.
1. Sürekli eğitilen ve tüm FastComments.com kiracıları arasında paylaşılan geleneksel bir Naïve-Bayes sınıflandırıcısı.

2. A traditional Naïve-Bayes classifier that is continuously trained, which is **isolated** to your tenant.
2. Sürekli eğitilen ve kiracınıza **izole** edilmiş geleneksel bir Naïve-Bayes sınıflandırıcısı.

3. Using ChatGPT 4.
3. ChatGPT 4'ün kullanılması.

Everyone has access to the shared and isolated Naïve-Bayes classifiers.

Herkes paylaşılan ve izole Naïve-Bayes sınıflandırıcılarına erişebilir.

The ChatGPT 4 option is selectable in the Comment Moderation Settings page if you are on Flex billing, since it bills based
on tokens used.

ChatGPT 4 seçeneği, Flex billing üzerindeyseniz Yorum Moderasyon Ayarları sayfasında seçilebilir; çünkü kullanılan token'lara göre faturalandırılır.

### Trust Factor

### Güven Faktörü

FastComments adjusts the spam filter for a user based on how much they are trusted for the given site.

FastComments, belirli bir site için kullanıcının ne kadar güvenilir olduğuna bağlı olarak spam filtresini ayarlar.

For example, if administrators have pinned lots of their comments, then probably they are a very trustworthy user. Or, if
they have been a member of the site for a long time and have a lot of comments, then their trust factor may be high as well.

Örneğin, yöneticiler kullanıcının birçok yorumunu sabitlediyse, muhtemelen çok güvenilir bir kullanıcıdır. Veya kullanıcı siteye uzun süredir üye olup çok sayıda yoruma sahipse, güven faktörü de yüksek olabilir.

### SSO

### SSO

Comments posted by SSO users can be considered spam, and will be checked as such. The exception is if the SSO user
has the same email as a tenant user who has one or more of the following permissions:

SSO kullanıcıları tarafından gönderilen yorumlar spam olarak değerlendirilebilir ve buna göre kontrol edilir. İstisna, SSO kullanıcısının e‑postası aşağıdaki izinlerden birine veya birkaçına sahip bir tenant kullanıcısıyla aynıysa geçerlidir:

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO users with these permissions will not have their comments checked for spam.

Bu izinlere sahip SSO kullanıcılarının yorumları spam için kontrol edilmeyecektir.

### Repeated Messages

### Tekrarlanan Mesajlar

FastComments will detect and prevent repeated messages. It will also detect repeated message that are very similar to help prevent spam. This cannot
be disabled as it prevents our platform from being used for abuse. If you have a high trust factor, this is taken into account when doing repeated message prevention.

FastComments tekrarlanan mesajları tespit eder ve engeller. Ayrıca spamı önlemeye yardımcı olmak için çok benzer olan tekrarlanan mesajları da tespit eder. Bu, platformumuzun suistimal amacıyla kullanılmasını önlediği için devre dışı bırakılamaz. Yüksek bir güven faktörünüz varsa, bu durum tekrarlanan mesajların engellenmesinde dikkate alınır.