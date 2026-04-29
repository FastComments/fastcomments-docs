**Şablon Kimliği:** `tos_enforcer`

Moderatör şablonu, manuel moderasyon yükünü azaltmayı hedefliyorsanız önerilen başlangıç noktasıdır. Yeni ve işaretlenmiş yorumları inceler ve topluluk kurallarınızı uygular.

### Yerleşik başlangıç istemi

[inline-code-attrs-start title = 'Moderatör Şablonu Başlangıç İstemi'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Bu istemi neredeyse her zaman sitenizin neye izin verip vermediğine dair somut örneklerle **zenginleştirmek (augment)** istersiniz. Platformun kendi tırmanma politikası (ban öncesi uyarı, banlamadan önce bellekte arama) zaten ajanın aldığı sistem istemine dahil edildiği için bunu tekrarlamanıza gerek yoktur.

### Tetikleyiciler

- **Yeni yorum gönderildi** (`COMMENT_ADD`) - temsilci her yeni yorumu inceler.
- **Yorum bir işaret eşik değerini geçti** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - temsilci diğer kullanıcıların işaretlediği bir yorumu yeniden değerlendirir.

### İzin verilen araçlar

- [`mark_comment_approved`](#tools-overview) - temsilcinin temiz yorumları yayınlayıp diğerlerini gizlediği ön-moderasyon yapan kiracılar için kullanışlı.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Yorum gönderemez, oy veremez, sabitleyemez, kilitleyemez, rozet veremez veya e-posta gönderemez — istem kasıtlı olarak dar tutulmuştur.

### Canlıya almadan önce önerilen eklemeler

- **[Topluluk Kuralları](#community-guidelines) belirleyin.** Birkaç cümlelik yazılı politika yeterlidir; temsilci her çalıştırmada bunu uygular.
- **`ban_user`'ı [onay](#approval-workflow) arkasına alın.** Bu, AB bölgesinde varsayılan olarak açıktır (bkz. [EU DSA Article 17 Compliance](#eu-dsa-compliance)) ve her yerde önerilir.
- **Düşük hacimli ama yüksek riskli içerikleriniz varsa `mark_comment_spam`'i de onay arkasına almayı düşünün.**
- **Ön-moderasyon çalıştırıyorsanız `mark_comment_approved`'ı onay arkasına alın.** Kötü bir yorumu onaylamak onu okurların önüne çıkarır; temsilci güven kazanana kadar bunu kısıtlayın.
- [Context Options](#context-options) içinde "Yorumcunun güven faktörü, hesap yaşı, ban geçmişi ve son yorumlarını dahil et" seçeneğini işaretleyin. Model, bir kullanıcının uzun süredir iyi niyetli olduğunu gördüğünde çok daha az agresif uyarı verir.

### Önerilen dry-run süresi

Bu şablonu etkinleştirmeden önce gerçek trafiğinizde en az bir hafta boyunca [dry-run](#dry-run-mode) modunda çalıştırın. Ayrıca önceki 30 güne karşı önizleme için [Test Runs (Replays)](#test-runs-replays) kullanın.

---