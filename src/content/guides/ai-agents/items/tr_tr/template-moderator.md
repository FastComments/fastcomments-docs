**Şablon Kimliği:** `tos_enforcer`

Moderator şablonu, manuel moderasyon yükünü azaltmayı hedefliyorsanız önerilen başlangıç noktasıdır. Yeni ve işaretlenmiş yorumları gözden geçirir ve topluluk kurallarınızı uygular.

Platformun kendi yükseltme politikası (yasaklamadan önce uyar, yasaklamadan önce belleği ara) ajanın aldığı sistem istemine zaten dahil edilmiştir, bu yüzden bunu tekrarlamanıza gerek yoktur.

### Tetikleyiciler

- **New comment posted** (`COMMENT_ADD`) - ajan her yeni yoruma bakar.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, varsayılan eşik: 3) - diğer kullanıcıların işaretlediği bir yorumu ajanın yeniden değerlendirmesini sağlar.

### İzin Verilen araçlar

- [`mark_comment_approved`](#tools-overview) - ajan temiz yorumları serbest bırakıp geri kalanını gizlediği ön moderasyon kiracıları için kullanışlıdır.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Yorum gönderemez, oy veremez, sabitleyemez, kilitleyemez, rozet veremez veya e-posta gönderemez - istem kasıtlı olarak dar tutulmuştur.

### Yayına alınmadan önce önerilen eklemeler

- **[Topluluk Kuralları](#community-guidelines) belirleyin.** Birkaç cümle yazılı politika yeterlidir; ajan her çalıştırmada bunu uygular.
- **`ban_user`'ı [onay](#approval-workflow) arkasına alın.** Bu, AB bölgesinde varsayılan olarak açıktır (bakınız [AB DSA Madde 17 Uyumluluğu](#eu-dsa-compliance)) ve her yerde önerilir.
- Düşük hacimli ancak yüksek riskli içerikleriniz varsa, `mark_comment_spam`'i de onay arkasına almayı düşünün.
- Ön moderasyon yürütüyorsanız `mark_comment_approved`'ı onay arkasına alın. Kötü bir yorumu onaylamak onu okuyucuların önüne koyar; ajan kuru deneme ile güven kazanana kadar bunu sınırlandırın.
- [Bağlam Seçenekleri](#context-options) içinde "Include commenter's trust factor, account age, ban history, and recent comments" seçeneğini işaretleyin. Model, birinin uzun süredir iyi niyetli bir kullanıcı olduğunu görebildiğinde çok daha az sert uyarıda bulunacaktır.

### Önerilen deneme süresi

Bu şablonu [deneme modu](#dry-run-mode) olarak, Etkinleştir'e geçmeden önce gerçek trafiğinizde en az bir hafta çalıştırın. Ayrıca son 30 güne karşı önizleme yapmak için [Test Çalıştırmaları (Replays)](#test-runs-replays) kullanın.

---