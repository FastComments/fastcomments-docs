FastComments otomatik olarak her yorum için ayrıntılı olayları izler ve moderasyon kararları ile sistem eylemlerine şeffaflık sağlar. Bu günlükler, bir yorumun neden onaylandığını, spam olarak işaretlendiğini veya durumunun değiştiğini anlamanıza yardımcı olur.

## Yorum Günlüklerine Erişim

Belirli bir yorumun günlüklerini görüntülemek için:

1. FastComments kontrol panelinizdeki **Moderate Comments** sayfasına gidin
2. İncelemek istediğiniz yorumu bulun
3. Yorumun eylem çubuğundaki **View Logs** düğmesine (saat simgesi) tıklayın
4. Bir iletişim kutusu açılacak ve bu yorum için tüm olay geçmişi gösterilecektir

Her günlük girişi şunları gösterir:
- **When** - Olayın zaman damgası
- **Who** - Olayı tetikleyen kullanıcı veya sistem (uygulanabilir olduğunda)
- **What** - Eylem veya olay türü
- **Details** - Önce/sonra değerleri, motor adları veya ilgili veriler gibi ek bağlam

## Yorum Günlüğü Olayları

Her yorum, yaşam döngüsü boyunca gerçekleşen olayların bir günlüğünü tutar. Aşağıda izlenen olay türleri yer almaktadır:

### Anonimleştirme Olayları
- **Anonymized** - Yorum içeriği temizlendi ve kullanıcı silinmiş olarak işaretlendi
- **RestoredFromAnonymized** - Yorum anonimleştirilmiş durumdan geri yüklendi

### Onay Olayları
- **ApprovedDueToPastComment** - Kullanıcı daha önce onaylanmış yorumlara sahip olduğu için yorum onaylandı (geçmiş yoruma referans içerir)
- **ApprovedIsAdmin** - Kullanıcı bir yönetici olduğu için yorum onaylandı
- **NotApprovedRequiresApproval** - Yorum manuel onay gerektiriyor
- **NotApprovedLowTrustFactor** - Düşük kullanıcı güven faktörü nedeniyle yorum onaylanmadı (güven faktörü değeri içerir)

### Profil Yorumu Onay Olayları
Bu olaylar özellikle kullanıcı profillerindeki yorumlar için geçerlidir:

- **ApprovedProfileAutoApproveAll** - Profil sahibi tüm yorumlar için otomatik onayı etkinleştirdiği için profil yorumu otomatik onaylandı
- **ApprovedProfileTrusted** - Yorumlayan güvenilir olduğu için profil yorumu onaylandı (güveni sağlayan yoruma referans içerir)
- **NotApprovedProfileManualApproveAll** - Profil sahibi manuel onayı etkinleştirdiği için profil yorumu manuel onay gerektiriyor
- **NotApprovedProfileNotTrusted** - Yorumlayan güvenilir olmadığı için profil yorumu onaylanmadı
- **NotApprovedProfileNewUser** - Yorumlayan yeni bir kullanıcı olduğu için profil yorumu onaylanmadı

### Spam Algılama Olayları
- **IsSpam** - Algılama motoru tarafından spam olarak işaretlendi (kararı veren motor belirtilir)
- **IsSpamDueToBadWords** - Küfür filtresi nedeniyle spam olarak işaretlendi
- **IsSpamFromLLM** - AI/LLM motoru tarafından spam olarak işaretlendi (motor adı, yanıt ve token sayısı içerir)
- **IsSpamRepeatComment** - Tekrarlayan olduğu için spam olarak işaretlendi (hangi motorun tespit ettiğini içerir)
- **NotSpamIsOnlyImage** - Yalnızca görseller içerdiği için spam olarak işaretlenmedi
- **NotSpamIsOnlyReacts** - Yalnızca tepkiler içerdiği için spam olarak işaretlenmedi
- **NotSpamNoLinkOrMention** - Şüpheli bağlantı veya bahsetme bulunmadığı için spam olarak işaretlenmedi
- **NotSpamPerfectTrustFactor** - Yüksek kullanıcı güveni nedeniyle spam olarak işaretlenmedi
- **NotSpamTooShort** - Analiz edilecek kadar kısa olduğu için spam olarak işaretlenmedi
- **NotSpamSkipped** - Spam kontrolü atlandı
- **NotSpamFromEngine** - Algılama motoru tarafından spam olmadığı belirlendi (motor adı ve güven faktörü içerir)

### Kötü Kelimeler/Argo Olayları
- **BadWordsCheckFailed** - Argo filtresi kontrolü bir hatayla karşılaştı
- **BadWordsFoundBadPhrase** - Argo filtresi uygunsuz bir ifade tespit etti (ifade içerir)
- **BadWordsFoundBadWord** - Argo filtresi uygunsuz bir kelime tespit etti (kelime içerir)
- **BadWordsNoDefinitionForLocale** - Yorum diline ait argo tanımlamaları bulunamadı (yerel ayar içerir)

### Kullanıcı Doğrulama Olayları
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Yorumun onaylanması için doğrulama gerekir ancak kullanıcı doğrulanmış oturumda değil
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Yorumun onaylanması için doğrulama gerekir ancak kullanıcı henüz doğrulanmadı
- **InVerifiedSession** - Yorumu gönderen kullanıcı doğrulanmış bir oturumda
- **SentVerificationEmailNoSession** - Doğrulama e-postası doğrulanmamış kullanıcıya gönderildi
- **SentWelcomeEmail** - Hoş geldin e-postası yeni kullanıcıya gönderildi

### Güven ve Güvenlik Olayları
- **TrustFactorChanged** - Kullanıcının güven faktörü değiştirildi (önceki ve sonraki değerler içerir)
- **SpamFilterDisabledBecauseAdmin** - Yönetici kullanıcı için spam filtreleme atlandı
- **TenantSpamFilterDisabled** - Tüm kiracı için spam filtreleme devre dışı bırakıldı
- **RepeatCommentCheckIgnored** - Tekrarlayan yorum kontrolü atlandı (neden içerir)
- **UserIsAdmin** - Kullanıcı yönetici olarak tanımlandı
- **UserIsAdminParentTenant** - Kullanıcı ana kiracı yöneticisi olarak tanımlandı
- **UserIsAdminViaSSO** - Kullanıcı SSO aracılığıyla yönetici olarak tanımlandı
- **UserIsMod** - Kullanıcı moderatör olarak tanımlandı

### Yorum Durumu Değişiklikleri
Durum değişikliği olayları önceki ve sonraki değerleri, ayrıca değişikliği yapan kullanıcıyı içerir:

- **ExpireStatusChanged** - Yorum süresi dolma durumu değiştirildi
- **ReviewStatusChanged** - Yorum inceleme durumu değiştirildi
- **SpamStatusChanged** - Yorum spam durumu güncellendi
- **ApproveStatusChanged** - Yorum onay durumu değiştirildi
- **TextChanged** - Yorum metni düzenlendi (önceki ve sonraki metin içerir)
- **VotesChanged** - Yorum oy sayıları güncellendi (detaylı oy dağılımı içerir)
- **Flagged** - Yorum kullanıcılar tarafından işaretlendi
- **UnFlagged** - Yorum işaretleri kaldırıldı

### Moderasyon Eylemleri
- **Pinned** - Yorum moderatör tarafından sabitlendi (kim tarafından sabitlendiği içerir)
- **UnPinned** - Yorum moderatör tarafından sabitlemesi kaldırıldı (kim tarafından kaldırıldığı içerir)

### Bildirim Olayları
- **CreatedNotifications** - Yorum için bildirimler oluşturuldu (bildirim sayısı içerir)
- **NotificationCreateFailure** - Bildirim oluşturulamadı
- **BadgeAwarded** - Yorum için kullanıcı rozetine sahip oldu (rozet adı içerir)

### Bahsetme ve Yanıt Bildirim Olayları
Bu olaylar, e-posta veya bildirim alacak kişiyi belirtir. Hiçbir şey gönderilmediğinde, Detaylar sütunu nedenini gösterir.

- **MentionEmailSent** - Yorumda bahsedilen kullanıcıya e-posta gönderildi
- **MentionEmailSkipped** - Bahsedilen kullanıcıya e-posta gönderilmedi (neden içerir)
- **MentionHeldForApproval** - Bahsetme e-postası yorum onaylanana kadar beklemede
- **MentionNotificationCreated** - Bahsedilen kullanıcı uygulama içi bildirim aldı
- **MentionNotificationSkipped** - Bahsedilen kullanıcı uygulama içi bildirim almadı (neden içerir)
- **ReplyEmailSent** - Yanıtlanan yorumun yazarı bu yanıt hakkında e-posta aldı
- **ReplyEmailSkipped** - Yanıtlanan yorumun yazarı e-posta almadı (neden içerir)
- **ReplyNotificationSkipped** - Yanıtlanan yorumun yazarı uygulama içi bildirim almadı (neden içerir)

Bir e-posta veya bildirim gönderilmediğinde gösterilen nedenler:

- Kullanıcı artık mevcut değil veya e-posta adresi yok
- Kullanıcı e-posta bildirimlerini kapattı veya o konu için bildirimleri kapattı
- İki kullanıcıdan biri diğerini engellemiş
- Kullanıcılar aynı SSO gruplarından hiçbiri içinde değil
- Kullanıcının e-posta adresi bir bounce veya spam şikayetinden sonra bastırma listesinde (bkz. [Email Suppression Management](/guide-notifications.html#email-suppression-management))
- Kullanıcının e-posta adresi example.com'da, bu adres e-posta alamaz
- Yorum spam olarak işaretlendi, silindi veya 7 gün içinde onaylanmadı
- Yanıtlanan yorum anonim olarak bırakıldı
- Kullanıcı kendi yorumuna yanıt verdi
- Kullanıcı yanıtta bahsedildi, bu yüzden yanıt e-postası yerine bahsetme e-postası aldı
- Kullanıcı zaten yorum için bir yanıt bildirimi almıştı
- Gönderim 5 kez başarısız oldu

Eğer teslimat başarısız olursa veya gönderim limitine ulaşırsa, e-posta yeniden denemek için kuyruğa alınır ve günlük girdisi bunu belirtir.

### Yayınlama Olayları
- **PublishedLive** - Yorum canlı abonelere yayınlandı (aboneler sayısı içerir)

### Entegrasyon Olayları
- **WebhookSynced** - Yorum webhook aracılığıyla senkronize edildi

### Spam Kuralı Olayları
- **SpamRuleMatch** - Yorum özel bir spam kuralıyla eşleşti (kural detayları içerir)

### Yerelleştirme Olayları
- **LocaleDetectedFromText** - Yorum metninden dil yerel ayarı otomatik olarak tespit edildi (tespit edilen dil ve yerel ayar içerir)

## Yorum Günlükleri için Kullanım Durumları

Yorum günlükleri otomatik olarak oluşturulur ve her yorumla birlikte saklanır. Aşağıdaki konularda değerli içgörüler sağlar:

- **Moderasyon kararlarını anlamak** - Bir yorumun neden onaylandığını, incelenmek üzere tutulduğunu veya spam olarak işaretlendiğini tam olarak görün
- **Onay/spam sorunlarını ayıklamak** - Yorumlar beklenildiği gibi davranmadığında karar mantığını izleyin
- **Kullanıcı davranış kalıplarını izlemek** - Güven faktörü değişikliklerini ve doğrulama durumunu izleyin
- **Moderatör eylemlerini denetlemek** - Moderatörlerin belirli yorumlar üzerindeki eylemlerini gözden geçirin
- **Spam filtresi etkinliğini araştırmak** - Hangi algılama motorlarının spam yakaladığını ve hangilerinin yakalamadığını görün
- **Entegrasyonları sorun gidermek** - Webhook senkronizasyonlarını ve bildirim teslimatını doğrulayın

Bu günlükler, moderasyon sürecinde şeffaflığı sürdürmeye yardımcı olur ve yorum sisteminizin davranışını ince ayarlamanıza yardımcı olur.