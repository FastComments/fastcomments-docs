---
Moderasyon işlemlerinin bir alt kümesi, Yorum Moderasyonu sayfasına gitmeden doğrudan yorum dizisinin kendisinden yapılabilir.

Giriş yaptığınızda, bir yorumun sağ üstündeki düzenle düğmesine tıklayın. Bir moderatör olarak aşağıdaki seçeneklere sahip olmalısınız:

- **Yorumu Sabitle**
- **Yorumu Sil**
- **Yorumu Sil** + **Kullanıcıyı Yasakla** (Kalıcı veya Gölge, ayrıntılar daha sonra)
- **Yorumu Düzenle**
- **Yorumu Kilitle** veya **Kilidini Aç** (aşağıda daha fazla detay)
- O yorumu **Onaylandı** (göster) veya **Onaylanmadı** (gizle) olarak işaretle
- O yorumu **Spam** veya **Spam Değil** olarak işaretle

### Bir Yorumu Kilitleme

Tek bir yorumu kilitlemek, yoruma yeni yanıtların gelmesini engeller ve ayrıca yorumun kendisinin kilit açılana kadar düzenlenmesini veya silinmesini engeller. Bu, yöneticiler ve moderatörler dahil herkes için geçerlidir. Kilitli bir yorumu düzenlemeniz veya kaldırmanız gerekiyorsa, önce kilidini açın, değişikliği yapın ve isterseniz yeniden kilitleyin.

Kilitlemiş bir yorumun sağ üst köşesinde, dizinin kapalı olduğunu okuyucuların hemen görmesi için bir kilit simgesi görünür. Kilitli yorumlarda Düzenle ve Sil menü girdileri hem yorum bileşeninde hem de herkese açık API'de gizlenir (`PATCH` ve `DELETE` kilitli bir yoruma karşı çağrıldığında `code: 'locked'` döndürür).

Kilidi atlayan iki kasıtlı istisna vardır, çünkü aksi halde geride yetim veri kalır: bir kullanıcı tüm hesabını sildiğinde (yorumları kilit durumuna bakılmaksızın temizlenir) ve bir moderatör "delete all comments from this user" seçeneğiyle bir kullanıcıyı yasakladığında (temizlik kilitleri kaldırır).

### Yorum Dizilerini Kapatma

Giriş yapmış moderatörler ve yöneticiler, yorum alanının üst kısmındaki üç nokta menüsünden `Close Thread` seçeneğini seçerek yorum dizilerini kilitleyebilir veya kapatabilir. Daha sonra istedikleri zaman `Re-Open Thread` seçeneğini seçerek yorumu yeniden açabilirler.

Bir yorum dizisini kapatmak yeni yorumları engeller, ancak oy kullanmaya ve kullanıcıların isterlerse kendi yorumlarını silmesine izin vermeye devam eder.

Yorum dizilerini kapatma ve yeniden açma, diziyi görüntüleyen tüm kullanıcıları anında etkiler.

Ayrıca o sayfa için özel bir özelleştirme kuralı oluşturarak bir diziyi salt okunur olarak işaretleyebilir ve böylece oy ve silme seçeneklerini de kaldırabilirsiniz.

### Gerçek Zamanlı Güncellemeler

Tüm bu işlemler, diğer kullanıcıların yorum dizilerini sayfayı yeniden yüklemelerine gerek kalmadan hemen güncelleyecektir. Ancak, bir yorumu gizleme veya spam olarak işaretleme gibi moderatör eylemleri, gerektiğinde hızlıca geri alınabilmeleri için yorumu **moderatörün** ekranından kaldırmaz. Yorumu gizlenmiş olarak belirtmek için, diğer yorumlara kıyasla vurgulanır (kaldırılma sebebine bağlı olarak vurgulama rengi değişir).

Örneğin, şu kullanıcılar verilsin: `A (commenter)`, `B (Moderator 1)`, ve `C (Moderator 2)`.

...ve aşağıdaki senaryo:

1. `User B (Moderator 1)` bir yorumu gizler.
2. `User A (commenter)` için o yorum hemen gizlenir.
3. `User C (Moderator 2)` için o yorum hemen gizlenir.
4. Değişikliği yapan kullanıcı olan `User B (Moderator 1)` için yorum ekranında kalır, ancak kaldırılmış olarak vurgulanır. Yaptıkları işlemi geri alma seçeneğine sahiptir; geri alındığında diğer kullanıcılar güncellemeyi tekrar canlı olarak göreceklerdir.

---