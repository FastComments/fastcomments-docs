Bir **AI Agent**, FastComments kiracınıza (tenant) kapsamlanmış, topluluğunuzdaki olayları izleyen ve sizin adınıza harekete geçen otonom bir çalışandır.

Her ajanın kontrol ettiğiniz üç şeyi vardır:

1. **Bir kişilik.** Tonu, rolü ve karar verme tarzını tanımlayan serbest metinli bir başlangıç istemi ("Sıcakkanlı bir topluluk karşılama görevlisisin", "Topluluk kurallarını uygularsın ancak yasaklamaya karşı uyarıya yönelirsin" vb.).
2. **Bir veya daha fazla tetikleyici.** Ajanı uyandıran olayların listesi - yeni bir yorum, bir yorumun oy veya bayrak eşiğini aşması, bir moderatör eylemi, bir kullanıcının sitedeki ilk yorumu ve diğerleri. Tam liste [Trigger Events Overview](#triggers-overview) bölümündedir.
3. **Bir araç izin listesi.** Ajanın yapmasına izin verilen şeyler - yorum gönderme, oy verme, sabitleme, kilitleme, spam olarak işaretleme, kullanıcı yasaklama, DM ile uyarma, rozet verme, e-posta gönderme, paylaşılan bir belleği kaydetme ve arama. Tam liste [Allowed Tool Calls Overview](#tools-overview) bölümündedir.

Bir tetikleyici tetiklendiğinde, ajan ne olduğunu açıklayan bir bağlam mesajı alır (yorum, sayfa, isteğe bağlı iş parçacığı/kullanıcı/sayfa bağlamı) ve başlangıç istemi ile topluluk kurallarınızla birlikte istemlendirilir. Ardından harekete geçmek için araçları çağırır ve her çağrıyla birlikte bir gerekçe ve bir güven puanı kaydeder.

### Ajanlar asenkron çalışır

Ajanlar **onları tetikleyen kullanıcı eylemini asla engellemez**. Bir okuyucu bir yorum gönderir, yorum kaydedilir ve iş parçacığında gösterilir, yanıt döndürülür ve ancak *ondan sonra* ajan üzerinde çalışır - hemen veya yapılandırılmış bir gecikmeden sonra (bkz. [Deferred Triggers](#trigger-deferred-delay)). Ajanın yaptığı hiçbir şey kullanıcıya yönelik deneyime gecikme eklemez.

### Neden kullanmalısınız

- **Ölçekte moderasyon yapın.** Bariz spam'leri işaretleyin ve kuyruğu 24 saat izlemeden tekrarlayan suçluları yasaklayın.
- **Yeni yorumcuları karşılayın.** İlk kez yorum yapanlara kendi sesinizle yanıt verin.
- **En iyi içeriği öne çıkarın.** Özlü üst düzey yorumları bir oy eşiğini aştıklarında sabitleyin.
- **Kurallarınızı tutarlı bir şekilde uygulayın.** Aynı politika metnini her sınırda yorum için uygulayın.
- **Uzun iş parçacıklarını özetleyin.** Çok sayfalı tartışmaların tarafsız özetlerini gönderin.

### Sizi kontrolde tutan şeyler

- **Deneme modu.** Her yeni ajan **Dry Run** modunda gelir: tetikleyicileri işler, modeli çalıştırır ve *yapacağını* kaydeder, ancak gerçek bir eylem almaz. Bkz. [Dry-Run Mode](#dry-run-mode).
- **Onaylar.** Herhangi bir eylem alt kümesi insan onayına tabi tutulabilir. Bkz. [Approval Workflow](#approval-workflow).
- **Ajan başına ve hesap başına bütçeler.** Sert günlük ve aylık sınırlar. Bkz. [Budgets Overview](#budgets-overview).
- **Araç izin listesi.** İzin verilmeyen araçlar modelin paletinden çıkarılır - ajan kelimenin tam anlamıyla onları talep edemez. Bkz. [Allowed Tool Calls Overview](#tools-overview).
- **Her eylemde denetim alanları.** Model bir gerekçe ve güven puanı içermelidir. Her ikisi de çalışma zaman çizelgesinde ve her onayda görünür. Bkz. [Run Detail View](#run-detail-view).
- **AB DSA Madde 17.** AB bölgesinde, tamamen otomatik yasaklamalar engellenir. Bkz. [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Verileriniz üzerinde eğitim yok.** FastComments, istemleriniz veya yorumlarınız üzerinde eğitim yapmayan sağlayıcılar kullanır.

### İnsan moderasyonuyla birlikte nasıl yer alırlar

Ajanlar ve insan moderatörler aynı yorum platformunu paylaşır: ajanlar eylemleri aynı kanallar üzerinden alır (onayla, spam, yasakla, rozet, sabitle, kilitle, yaz) ve bu eylemler aynı [Comment Logs](/guide-moderation.html#comment-logs), aynı [Moderation Page](/guide-moderation.html#moderate-comments-page) ve aynı bildirim akışlarında görünür. Ajanlar ve insanlar birbirlerinin işini görür ve birbirlerine tepki verebilir - moderatör eylemlerinin kendileri geçerli ajan tetikleyicileridir (bkz. [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) ve benzerleri).

Kuyrukta insanlara da ihtiyacınız varsa, FastComments [Human Moderation Services](/guide-moderation.html#human-moderation-services) sunar: moderatörlerimiz, kurallarınız altında, panonuzda, ajanlarınızla birlikte çalışır.