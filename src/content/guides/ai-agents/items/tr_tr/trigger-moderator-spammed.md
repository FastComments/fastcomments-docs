Bir moderatör bir yorumu spam olarak işaretlediğinde tetiklenir.

### Ajanın aldığı bağlam

- Yorum, işlem sonrası `Is Spam` bayrağı ile.
- **tetikleyen kullanıcı kimliği** - işlemi yapan moderatör.
- Yapılandırıldığı şekilde isteğe bağlı konu dizisi / kullanıcı geçmişi / sayfa bağlamı.

### Bunu kim tetikler

Bu bir insan moderatör eylemidir. Ajan kaynaklı spam işaretlemeleri (aracılığıyla [`mark_comment_spam`](#tools-overview)) bu tetikleyiciyi **tetiklemez**.

### Yaygın kullanımlar

- **Bellek kaydı** - bir ajanın spamlenen kullanıcı hakkında bir hafıza notu kaydetmesini sağlayın (ör. "daha önce moderatör tarafından X nedeniyle spamlenmiş") böylece gelecekteki moderasyon ajanlarının bağlamı olur.
- **Kullanıcı düzeyinde yaptırım** - bir moderatörün bir yorumu spam olarak işaretlemesi, bir ajanın onaya tabi olacak şekilde ek olarak uyarı veya kısa süreli yasaklama uygulaması için işaret olabilir.
- **Harici sistem aynası** aracılığıyla [Webhooks](#webhooks-overview).

---