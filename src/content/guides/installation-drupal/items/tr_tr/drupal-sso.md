FastComments, SSO (tek oturum açma) aracılığıyla Drupal'ın kullanıcı sistemiyle bütünleşir. Kullanıcılarınız Drupal sitenize giriş yapar ve modül onların kimliğini FastComments'a otomatik olarak iletir. Ek hesap oluşturmanıza veya başlangıç senkronizasyonu çalıştırmanıza gerek yoktur.

The module supports three SSO modes, set under `Administration > Configuration > Content > FastComments`.

### Yok

SSO yok. Kullanıcılar misafir olarak yorum yapar veya bir FastComments hesabı oluşturur. Siteniz herkese açıksa ve yorumları Drupal kullanıcılarına bağlamanız gerekmiyorsa bunu kullanın.

### Basit

Drupal kullanıcısının adını, e-postasını ve avatarını sunucu tarafı doğrulaması olmadan FastComments'a aktarır. API Secret gerekmez. Dahili veya düşük riskli siteler için uygundur.

### Güvenli (önerilir)

Her kullanıcı kimliğini FastComments ile doğrulamak için [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) kullanır. API Secret yapılandırıldığında tercih etmeniz gereken mod budur ve bir ziyaretçinin başka bir kullanıcının kimliğine bürünmesini engelleyen tek moddur.

Kullanıcı kimliği, bir kullanıcı yorum dizisini her görüntülediğinde FastComments'a iletilir. Çalıştırılması gereken bir başlangıç veya sürekli senkronizasyon yoktur.

<sup>(İsteğe bağlı)</sup> Yöneticilerinizi [Kullanıcılar ve Yöneticiler](https://fastcomments.com/auth/my-account/users) bölümüne ve moderatörleri [Yorum Moderatörleri](https://fastcomments.com/auth/my-account/moderate-comments/moderators) bölümüne ekleyin; bu, deneyimlerini iyileştirir ve moderatörler için istatistik takibini etkinleştirir.

SSO'nin nasıl çalıştığına dair daha detaylı bilgi için özelleştirme belgelerinin [SSO bölümü](/guide-customizations-and-configuration.html#sso)'ne bakın.