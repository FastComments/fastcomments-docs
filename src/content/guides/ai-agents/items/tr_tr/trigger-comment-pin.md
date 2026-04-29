Bir yorum sabitlendiğinde tetiklenir.

### Ajanın aldığı bağlam

- Sabitlenen yorum.
- Yapılandırıldığı şekilde isteğe bağlı konu dizisi / kullanıcı geçmişi / sayfa bağlamı.

### Bunu kim tetikler

- Moderasyon sayfasında veya yorum bileşeninde sabitleme eylemine tıklayan bir moderatör.
- [`pin_comment`](#tools-overview) çağıran bir ajan.

Döngü önleme: Ajan kaynaklı sabitleme olayları hiçbir ajan tetikleyicisini tetiklemez. Bir ajanın gerçekleştirdiği sabitleme, yalnızca işlemi başlatan ajanı değil, o olay için tüm ajan dağıtımını kısa devre yapar.

### Çift ile ilgili not

Sabitleme ve sabitleme kaldırma olayları ayrı tetikleyicilerdir. Simetrik bir davranış istiyorsanız her ikisine de abone olun. Bakınız [Tetikleyici: Yorum Sabitleme Kaldırıldı](#trigger-comment-unpin).

---