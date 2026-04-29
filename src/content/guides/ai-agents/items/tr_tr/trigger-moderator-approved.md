---
Bir moderatör bir yorumu onayladığında tetiklenir.

### Ajanın aldığı bağlam

- Yeni onaylanan yorum.
- **tetikleyen kullanıcı kimliği** - yorumu onaylayan moderatör.
- Yapılandırmaya bağlı olarak isteğe bağlı konu / kullanıcı geçmişi / sayfa bağlamı.

### Bunu kim tetikler

Bir insan moderatör tarafından yapılan işlem.

### Önemli

- Bir "onaylanmış" yorum FastComments terminolojisinde **görünür** bir yorumdur. Onaylanmış/onaylanmamış ve incelenmiş/incelenmemiş ayrımını görmek için moderasyon kılavuzundaki [Onaylama Nasıl Çalışır](/guide-moderation.html#moderation-approvals) bölümüne bakın.
- Tetik, onay **geçişlerinde** tetiklenir: onaylanmamış bir yorumun onaylanmaya geçmesi tetikler; zaten onaylı olan bir yorumun tekrar kaydedilmesi tetiklemez.
- Yorumların varsayılan olarak otomatik onaylandığı kiracılar için, bu tetik yalnızca bir moderatör daha önce gizlenmiş bir yorumu açıkça tekrar onayladığında çalışır.

### Yaygın kullanımlar

- **Karşılama / etkileşim** - bir ajan, bir moderatör onları onayladığı anda, gönderi zamanı yerine ilk kez yorum yapanlara yanıt verebilir.
- **Ajanlar arası koordinasyon** - ayrı bir ajan yorumu inceleme için işaretlemişse, onay insan incelemesinin bittiğine dair işarettir.
- **Denetim izi** [Webhooks](#webhooks-overview) aracılığıyla.

---