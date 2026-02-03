FastComments, moderasyon kararları ve sistem eylemlerinde şeffaflık sağlamak için her yorum için ayrıntılı olayları otomatik olarak izler. Bu günlükler, bir yorumun neden onaylandığını, spam olarak işaretlendiğini veya durumunun neden değiştirildiğini anlamanıza yardımcı olur.

## Yorum Günlüklerine Erişim

Belirli bir yorumun günlüklerini görüntülemek için:

1. FastComments panelinizde **Moderate Comments** sayfasına gidin
2. İncelemek istediğiniz yorumu bulun
3. Yorumun işlem çubuğundaki **View Logs** düğmesine (saat simgesi) tıklayın
4. O yorum için gerçekleşen olayların tam geçmişini gösteren bir iletişim kutusu görünecektir

Her günlük girişi şunları gösterir:
- **When** - Olayın zaman damgası
- **Who** - Olayı tetikleyen kullanıcı veya sistem (uygulandığında)
- **What** - Eylem veya olay türü
- **Details** - Önce/sonra değerleri, motor adları veya ilgili veriler gibi ek bağlam

## Yorum Günlüğü Olayları

Her yorum, yaşam döngüsü sırasında gerçekleşen olayların bir günlüğünü tutar. Aşağıda izlenen olay türleri yer almaktadır:

### Anonimleştirme Olayları
- **Anonymized** - Yorum içeriği temizlendi ve kullanıcı silinmiş olarak işaretlendi
- **RestoredFromAnonymized** - Yorum anonimleştirilmiş durumdan geri yüklendi

### Onay Olayları
- **ApprovedDueToPastComment** - Kullanıcının daha önce onaylanmış yorumları olduğu için yorum onaylandı (geçmiş yoruma referans içerir)
- **ApprovedIsAdmin** - Kullanıcı yönetici olduğu için yorum onaylandı
- **NotApprovedRequiresApproval** - Yorum manuel onay gerektiriyor
- **NotApprovedLowTrustFactor** - Düşük kullanıcı güven faktörü nedeniyle yorum onaylanmadı (güven faktörü değerini içerir)

### Profil Yorum Onay Olayları

Bu olaylar özellikle kullanıcı profillerindeki yorumlara uygulanır:

- **ApprovedProfileAutoApproveAll** - Profil sahibi tüm yorumlar için otomatik onayı etkinleştirdiği için profil yorumu otomatik onaylandı
- **ApprovedProfileTrusted** - Profil yorumu, yorum yapan kişi güvenilir olduğu için onaylandı (güveni sağlayan yoruma referans içerir)
- **NotApprovedProfileManualApproveAll** - Profil sahibi manuel onayı etkinleştirdiği için profil yorumu manuel onay gerektiriyor
- **NotApprovedProfileNotTrusted** - Profil yorumu, yorum yapan kişi güvenilir olmadığı için onaylanmadı
- **NotApprovedProfileNewUser** - Profil yorumu, yorum yapan kişinin yeni kullanıcı olması nedeniyle onaylanmadı

### Spam Tespiti Olayları
- **IsSpam** - Yorum tespit motoru tarafından spam olarak işaretlendi (kararı veren motoru içerir)
- **IsSpamDueToBadWords** - Yorum, küfür/argo filtresi nedeniyle spam olarak işaretlendi
- **IsSpamFromLLM** - Yorum, AI/LLM motoru tarafından spam olarak işaretlendi (motor adı, yanıt ve token sayısını içerir)
- **IsSpamRepeatComment** - Tekrarlayıcı olduğu için yorum spam olarak işaretlendi (hangi motorun tespit ettiğini içerir)
- **NotSpamIsOnlyImage** - Yorum yalnızca resim içerdiği için spam olarak işaretlenmedi
- **NotSpamIsOnlyReacts** - Yorum yalnızca tepkiler içerdiği için spam olarak işaretlenmedi
- **NotSpamNoLinkOrMention** - Şüpheli bağlantı veya mention olmadığı için yorum spam olarak işaretlenmedi
- **NotSpamPerfectTrustFactor** - Yüksek kullanıcı güveni nedeniyle yorum spam olarak işaretlenmedi
- **NotSpamTooShort** - Analiz etmek için çok kısa olduğu için yorum spam olarak işaretlenmedi
- **NotSpamSkipped** - Spam kontrolü atlandı
- **NotSpamFromEngine** - Tespit motoru tarafından spam olmadığı belirlendi (motor adı ve güven faktörünü içerir)

### Küfür/Kötü Sözcük Olayları
- **BadWordsCheckFailed** - Küfür filtresi kontrolü sırasında hata oluştu
- **BadWordsFoundBadPhrase** - Küfür filtresi uygunsuz bir ifade tespit etti (ifadeyi içerir)
- **BadWordsFoundBadWord** - Küfür filtresi uygunsuz bir kelime tespit etti (kelimeyi içerir)
- **BadWordsNoDefinitionForLocale** - Yorum diline ait küfür tanımları bulunamadı (lokali içerir)

### Kullanıcı Doğrulama Olayları
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Yorum onaylanması için doğrulama gerekiyor ancak kullanıcı doğrulanmış oturumda değil
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Yorum onaylanması için doğrulama gerekiyor ancak kullanıcı henüz doğrulanmamış
- **InVerifiedSession** - Yorum yapan kullanıcı doğrulanmış bir oturumda
- **SentVerificationEmailNoSession** - Doğrulama e-postası doğrulanmamış kullanıcıya gönderildi
- **SentWelcomeEmail** - Yeni kullanıcıya hoş geldin e-postası gönderildi

### Güven ve Güvenlik Olayları
- **TrustFactorChanged** - Kullanıcının güven faktörü değiştirildi (önce ve sonra değerlerini içerir)
- **SpamFilterDisabledBecauseAdmin** - Yönetici kullanıcısı için spam filtrelemesi atlandı
- **TenantSpamFilterDisabled** - Tüm tenant için spam filtrelemesi devre dışı bırakıldı
- **RepeatCommentCheckIgnored** - Tekrarlayan yorum kontrolü atlandı (sebebi içerir)
- **UserIsAdmin** - Kullanıcı yönetici olarak tespit edildi
- **UserIsAdminParentTenant** - Kullanıcı parent tenant yöneticisi olarak tespit edildi
- **UserIsAdminViaSSO** - Kullanıcı SSO ile yönetici olarak tespit edildi
- **UserIsMod** - Kullanıcı moderatör olarak tespit edildi

### Yorum Durumu Değişiklikleri

Durum değişikliği olayları önce ve sonra değerlerini ile değişikliği yapan kullanıcıyı içerir:

- **ExpireStatusChanged** - Yorumun sonlandırma durumu değiştirildi
- **ReviewStatusChanged** - Yorum inceleme durumu değiştirildi
- **SpamStatusChanged** - Yorum spam durumu güncellendi
- **ApproveStatusChanged** - Yorum onay durumu değiştirildi
- **TextChanged** - Yorum metin içeriği düzenlendi (önce ve sonra metni içerir)
- **VotesChanged** - Yorum oy sayıları güncellendi (detaylı oy dökümünü içerir)
- **Flagged** - Yorum kullanıcılar tarafından işaretlendi
- **UnFlagged** - Yorum işaretleri kaldırıldı

### Moderasyon Eylemleri
- **Pinned** - Yorum moderatör tarafından sabitlendi (kim tarafından sabitlendiğini içerir)
- **UnPinned** - Yorum moderatör tarafından sabitlemesi kaldırıldı (kim tarafından kaldırıldığını içerir)

### Bildirim Olayları
- **CreatedNotifications** - Yorum için bildirimler oluşturuldu (bildirim sayısını içerir)
- **NotificationCreateFailure** - Bildirim oluşturma başarısız oldu
- **BadgeAwarded** - Yorum için kullanıcıya rozet verildi (rozet adını içerir)

### Yayınlama Olayları
- **PublishedLive** - Yorum canlı abonelere yayınlandı (abone sayısını içerir)

### Entegrasyon Olayları
- **WebhookSynced** - Yorum webhook ile senkronize edildi

### Spam Kuralı Olayları
- **SpamRuleMatch** - Yorum özel bir spam kuralı ile eşleşti (kural detaylarını içerir)

### Yerelleştirme Olayları
- **LocaleDetectedFromText** - Yorum metninden dil lokali otomatik olarak tespit edildi (tespit edilen dil ve lokali içerir)

## Yorum Günlükleri için Kullanım Durumları

Yorum günlükleri her yorumla birlikte otomatik olarak oluşturulur ve saklanır. Sağladıkları değerler:

- **Understanding moderation decisions** - Bir yorumun neden onaylandığını, incelemeye alındığını veya spam olarak işaretlendiğini tam olarak görün
- **Debugging approval/spam issues** - Yorumlar beklenildiği gibi davranmadığında karar mantığını izleyin
- **Tracking user behavior patterns** - Güven faktörü değişikliklerini ve doğrulama durumunu izleyin
- **Auditing moderator actions** - Moderatörlerin belirli yorumlar üzerinde hangi eylemleri yaptığını gözden geçirin
- **Investigating spam filter effectiveness** - Hangi tespit motorlarının spam yakaladığını ve hangilerinin yakalayamadığını görün
- **Troubleshooting integrations** - Webhook senkronizasyonlarını ve bildirim teslimatını doğrulayın

Bu günlükler, moderasyon sürecinde şeffaflığı korumaya yardımcı olur ve yorum sisteminizin davranışını hassaslaştırmada destek sağlar.