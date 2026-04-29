Bir yorum kilitlendiğinde tetiklenir.

### Ajanın aldığı bağlam

- Kilitlenmiş yorum.
- Yapılandırıldığı şekilde isteğe bağlı başlık / kullanıcı geçmişi / sayfa bağlamı.

### Bunu kim tetikler

- Moderasyon sayfasında veya yorum bileşeninde kilitleme eylemini kullanan bir moderatör.

### Yaygın kullanımlar

- **İnceleyicileri bilgilendir** - bir kilitleme olayı genellikle hararetli bir başlığı izler; moderasyon Slack kanalınıza gönderilecek bir webhook, insanlara geri kalanını devralma imkanı verebilir.
- **Soğuma süresi uygulaması** - kilitlemeden 24 saat sonra kilidin açılıp açılmayacağını değerlendirecek ayrı bir ajan üzerinde bir [ertelenmiş tetikleyici](#trigger-deferred-delay) zamanlayın.

### Eşleşen tetikleyici

Ayna tetikleyici için bkz. [Tetikleyici: Yorumun kilidi açıldı](#trigger-comment-unlock).

---