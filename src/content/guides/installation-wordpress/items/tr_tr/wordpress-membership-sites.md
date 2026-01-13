FastComments, SSO (single-sign-on) adı verilen yöntemle yalnız üyelikli sitelerle çalışır. Üyeleriniz WordPress sitenize giriş yapar, ancak
yorum yapmak için FastComments ile hesap oluşturmak veya sosyal medya ile giriş yapmak zorunda kalmazlar. Üyeleriniz (yöneticiler dahil) WordPress sitenize giriş yaptıysalar,
yorum yapabilirler. Yöneticileriniz ve moderatörleriniz de yorumları doğrudan WordPress blog yazılarınız üzerinden yönetebileceklerdir.

<sup>(İsteğe Bağlı)</sup> Yöneticilerinizi ayrıca [Kullanıcılar & Yöneticiler](https://fastcomments.com/auth/my-account/users) sayfasına ve moderatörleri [Yorum Moderatörleri](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
sayfasına eklemeyi unutmayın; bu, deneyimlerini iyileştirir ve moderatörler için istatistik takibini etkinleştirir.

SSO, eklenti panosuna gidip "SSO Settings"e tıklayarak etkinleştirilebilir.

Öncelikle sitenizin "Herkes Kayıt Olabilir" özelliğini etkinleştirmeniz gerekecek.

Tüm kullanıcı bilgileri, bir kullanıcı bir yorum dizisini görüntülediğinde [HMAC](https://en.wikipedia.org/wiki/HMAC) aracılığıyla sorunsuz ve güvenli bir şekilde FastComments'e aktarılır.

Üyelerinizi FastComments sunucularına kopyalamak için çalıştırılması gereken bir başlangıç veya sürekli senkronizasyon yoktur. Bu, bir blog yazısını açıp yorum dizilerini görüntülediklerinde otomatik olarak yapılır.

## İsimler ve Avatarlar

Eklenti, kullanıcı herhangi bir yorum dizisini görüntülediğinde kullanıcının görüntüleme adını ve avatarını FastComments içindeki tüm yorumlarında otomatik olarak güncelleyecektir. Avatarlar Gravatar veya eklentinin `get_avatar_url` çağracağı WordPress içindeki herhangi bir avatar yönetimi eklentisi aracılığıyla desteklenir.

---