---
Bir moderatör bir yorumu incelendi olarak işaretlediğinde tetiklenir.

### Ajanın aldığı bağlam

- Yorum.
- **triggering user ID** - yorumu inceleyen moderatör.
- Yapılandırıldığı şekilde isteğe bağlı konu / kullanıcı geçmişi / sayfa bağlamı.

### Bunu kim tetikler

Moderasyon sayfasında, yorum bileşeninde veya API aracılığıyla gerçekleştirilen bir insan moderatör işlemi.

### Yaygın kullanım alanları

- **Denetim iletimi** via [Webhooks](#webhooks-overview).
- **Bellek yazmaları** - bu yorumun insan tarafından incelendiğine dair bir bellek notu kaydedin, böylece diğer ajanlar onu çift işlememiş olur.

### Dikkate değer

- "Reviewed" is one of the moderation queue states tracked separately from "approved" and "spam". A comment can be approved-and-reviewed, approved-but-not-reviewed, etc. See [Onayların Nasıl Çalıştığı](/guide-moderation.html#moderation-approvals) in the moderation guide.
- Bu tetikleyici, çok sayıda moderatöre sahip tenants'larda yüksek sıklıkta tetiklenir. Abone olurken seçici davranın ve buna göre bütçe ayırın.

---