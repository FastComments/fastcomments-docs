FastComments, her yorum için ayrıntılı olayları otomatik olarak izler ve bu sayede moderasyon kararları ve sistem eylemleri konusunda şeffaflık sağlar. Bu günlükler, bir yorumun neden onaylandığını, spam olarak işaretlendiğini veya durumunun neden değiştirildiğini anlamanıza yardımcı olur.

Belirli bir yorumu seçerek Yorumları Denetleme panosunda bireysel yorumların günlüklerini görüntüleyebilirsiniz.

## Yorum Günlüğü Olayları

Her yorum, yaşam döngüsü sırasında gerçekleşen olayların bir günlüğünü tutar. Aşağıda izlenen olay türleri verilmiştir:

### Anonimleştirme Olayları
- **Anonymized** - Yorum içeriği temizlendi ve kullanıcı silinmiş olarak işaretlendi

### Onay Olayları
- **ApprovedDueToPastComment** - Kullanıcının daha önce onaylanmış yorumları olduğu için yorum onaylandı
- **ApprovedIsAdmin** - Kullanıcının yönetici olması nedeniyle yorum onaylandı
- **NotApprovedRequiresApproval** - Yorum manuel onay gerektiriyor

### Spam Tespit Olayları
- **IsSpam** - Yorum tespit motoru tarafından spam olarak işaretlendi
- **IsSpamDueToBadWords** - Küfür filtresi nedeniyle yorum spam olarak işaretlendi
- **IsSpamFromLLM** - AI/LLM motoru tarafından yorum spam olarak işaretlendi
- **IsSpamRepeatComment** - Tekrarlayıcı olduğu için yorum spam olarak işaretlendi
- **NotSpamIsOnlyImage** - Sadece görseller içerdiği için yorum spam olarak işaretlenmedi
- **NotSpamIsOnlyReacts** - Sadece reaksiyonlar içerdiği için yorum spam olarak işaretlenmedi
- **NotSpamNoLinkOrMention** - Şüpheli bağlantı veya bahsetme olmadığı için yorum spam olarak işaretlenmedi
- **NotSpamPerfectTrustFactor** - Yüksek kullanıcı güveni nedeniyle yorum spam olarak işaretlenmedi
- **NotSpamTooShort** - Analiz için çok kısa olduğu için yorum spam olarak işaretlenmedi
- **NotSpamSkipped** - Spam kontrolü atlandı
- **NotSpamFromEngine** - Tespit motoru tarafından yorumun spam olmadığı belirlendi

### Kötü Kelimeler/Küfür Olayları
- **BadWordsCheckFailed** - Küfür filtresi kontrolünde hata oluştu
- **BadWordsFoundBadPhrase** - Küfür filtresi uygunsuz ifade tespit etti
- **BadWordsFoundBadWord** - Küfür filtresi uygunsuz kelime tespit etti
- **BadWordsNoDefinitionForLocale** - Yorumun dili için küfür tanımları yok

### Kullanıcı Doğrulama Olayları
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Yorum onaylanması için doğrulama gerekiyor ancak kullanıcı doğrulanmış oturumda değil
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Yorum onaylanması için doğrulama gerekiyor ancak kullanıcı henüz doğrulanmamış
- **InVerifiedSession** - Yorumu gönderen kullanıcı doğrulanmış oturumda
- **SentVerificationEmailNoSession** - Doğrulama e-postası doğrulanmamış kullanıcıya gönderildi
- **SentWelcomeEmail** - Yeni kullanıcıya karşılama e-postası gönderildi

### Güven ve Güvenlik Olayları
- **TrustFactorChanged** - Kullanıcının güven faktörü değiştirildi
- **SpamFilterDisabledBecauseAdmin** - Yönetici kullanıcı için spam filtrelemesi atlandı
- **TenantSpamFilterDisabled** - Tüm tenant için spam filtrelemesi devre dışı bırakıldı
- **RepeatCommentCheckIgnored** - Tekrarlayan yorum kontrolü atlandı
- **UserIsAdmin** - Kullanıcı yönetici olarak tanımlandı
- **UserIsAdminParentTenant** - Kullanıcı ana tenant yöneticisi olarak tanımlandı
- **UserIsAdminViaSSO** - Kullanıcı SSO aracılığıyla yönetici olarak tanımlandı
- **UserIsMod** - Kullanıcı moderatör olarak tanımlandı

### Yorum Durumu Değişiklikleri
- **ExpireStatusChanged** - Yorumun sona erme durumu değiştirildi
- **ReviewStatusChanged** - Yorum inceleme durumu değiştirildi
- **SpamStatusChanged** - Yorumun spam durumu güncellendi
- **ApproveStatusChanged** - Yorum onay durumu değiştirildi
- **TextChanged** - Yorum metin içeriği düzenlendi
- **VotesChanged** - Yorum oy sayıları güncellendi
- **Flagged** - Yorum kullanıcılar tarafından işaretlendi
- **UnFlagged** - Yorum işaretleri kaldırıldı

### Moderasyon Eylemleri
- **Pinned** - Yorum moderatör tarafından sabitlendi
- **UnPinned** - Yorumun sabitliği moderatör tarafından kaldırıldı
- **RestoredFromAnonymized** - Yorum anonimleştirilmiş durumdan geri alındı

### Bildirim Olayları
- **CreatedNotifications** - Yorum için bildirimler oluşturuldu
- **NotificationCreateFailure** - Bildirim oluşturma başarısız oldu
- **BadgeAwarded** - Yorum için kullanıcıya rozet verildi

### Yayınlama Olayları
- **PublishedLive** - Yorum canlı abonelere yayınlandı

### Entegrasyon Olayları
- **WebhookSynced** - Yorum webhook aracılığıyla senkronize edildi

### Spam Kuralı Olayları
- **SpamRuleMatch** - Yorum özel bir spam kuralıyla eşleşti

## Yorum Günlüklerine Erişim

Yorum günlükleri otomatik olarak oluşturulur ve her yorumla birlikte saklanır. Bunlar şu konularda değerli içgörüler sağlar:

- Moderasyon kararlarını anlamak
- Onay/spam sorunlarını hata ayıklamak
- Kullanıcı davranış kalıplarını izlemek
- Sistem eylemlerini denetlemek

Bu günlükler moderasyon sürecinde şeffaflığı korumaya yardımcı olur ve yorum sisteminizin davranışını ince ayarlamanıza yardımcı olur.