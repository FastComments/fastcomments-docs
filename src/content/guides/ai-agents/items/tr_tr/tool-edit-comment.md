---
Düzenle aracı, ajanın mevcut bir yorumun metnini değiştirmesine olanak tanır. Çoğu diğer moderasyon aracının yapmadığı şekilde tahrip edicidir: kullanıcı tarafından yazılan içeriğin üzerine yazar. Bunu dar ve net vakalar için ayırın.

### Neler yapar

Ajan bir yorum kimliği ve bir değiştirme gövdesi iletir. Platform yeni metni yoruma yazar ve hem eski metni hem de yeni metni yakalayan bir `TextChanged` kaydını yorumun denetim günlüğüne kaydeder. Orijinal asla kaybolmaz - moderatörler ajanın düzenlemeden önce yorumun ne dediğini okuyabilir.

Değiştirme, insan düzenlemesiyle aynı işleme hattından geçer: küfür maskeleme, mention çözümleme, hashtag çıkarımı ve bağlantı/görsel işlemleri, orijinal yazarın yeni metni gönderdiği durumla tamamen aynı şekilde davranır.

### Kapsam

Her yorum-değiştiren araç gibi, Edit tetikleyicinin izin listesinin (allowlist) kapsamıyla sınırlıdır - ajan yalnızca tetikleyicinin tetiklendiği yorumu, onun parent'ını veya aynı tetik bağlamından kapsam içi başka bir yorumu düzenleyebilir. A prompt-injection attempt to "edit comment XYZ" where XYZ is unrelated will be refused server-side before the executor runs.

### Döngüler

Ajan bir yorumu düzenlediğinde, platform insan düzenlemesinde olduğu gibi bir `COMMENT_EDIT` tetikleyicisi tetikler, ancak **diğer ajanlara dağıtımı bastırır**. Bu, her ikisi de `COMMENT_EDIT` dinleyen iki ajanın birbirlerinin düzenlemeleri üzerinde ping-pong yapmasını engeller.

### Ne zaman izin verilmeli

PII (kişisel olarak tanımlanabilir bilgiler) kırpmasını (redaction) yapan ajanlar için veya kendi kendini düzenleyen özetleyici/digest ajanlar için uygundur. Çoğu moderasyon ajanının bu araca **ihtiyacı yoktur** - mark-spam, warn, and ban tipik yaşam döngüsünü kapsar.

### Onaylar

**Onay gerektirecek şekilde sınırlandırmayı güçlü şekilde düşünün**, özellikle ajana güven inşa ederken. Bir ajanın bir kullanıcının sözlerini yeniden yazması, topluluğun fark edeceği ve tepki vereceği türden bir eylemdir ve itibar bakımından geri almak ("undo") bir silmeden daha zordur.

### Ayrıca bakınız

- [Trigger: Comment Edited](#trigger-comment-edit) - bir yorumun metni değiştiğinde tetiklenen tetikleyici.
- [Approval Workflow](#approval-workflow) - aracı insan incelemesi arkasına nasıl koyacağınız.

---