---
Bir yorum silindiğinde tetiklenir.

### Ajanın aldığı bağlam

- Yeni silinmiş yorum - metin, yazar, sayfa.
- Yapılandırıldığı şekilde isteğe bağlı konu / kullanıcı geçmişi / sayfa bağlamı.

### Önemli

- Hem **yumuşak silinmelerde** (yorum gizlenir ancak denetim için saklanır) hem de **sert silinmelerde** (yorum tamamen kaldırılır) tetiklenir. Tetikleyici işleyicisi yorumu kademeli silme iş akışından çözer; ajanın gördüğü son bilinen durumdur.
- Bir yorum tamamen silindikten sonra, o yorum ID'sini hedef alan araçlar (`pin_comment`, `mark_comment_spam`, vb.) başarısız olur.

### Yaygın kullanımlar

- **[Webhooks](#webhooks-overview) üzerinden denetim iletimi** - bir `trigger.succeeded` olayı yayılarak harici bir sistemin neyin silindiğini kaydetmesini sağlar.
- **Belleğe yazma** - ajanın bir silinme desenine dair bir [hafıza notu](#tools-overview) kaydetmesini sağlayın (silinen yorum kullanıcının 24 saatteki üçüncü yorumuydı, vb.).
- **Konular arası etkiler** - bir silme işleminin ajanın daha önce özetlediği bir konu yapısını değiştirdiğini fark edin ve yeniden özetleyip özetlememeyi değerlendirin.

### İşletme maliyeti notu

Eğer yüksek silme hacmine sahip bir siteniz varsa (yoğun insan moderasyonu), bu tetikleyici sık sık çalışabilir.

---