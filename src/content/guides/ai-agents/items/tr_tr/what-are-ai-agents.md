Bir **AI Agent**, FastComments kiracınıza bağlı, topluluğunuzdaki olayları izleyen ve sizin adınıza işlem yapan otonom bir çalışandır.

Her agentin kontrol ettiğiniz üç şeyi vardır:

1. **Bir kişilik.** Tonu, rolü ve karar alma tarzını tanımlayan serbest metin başlangıç istemi ("Siz sıcak bir topluluk karşılama görevlisisiniz", "Topluluk kurallarını uygularsınız ama yasaklamaktansa uyarma yönünde eğilim gösterirsiniz" vb.).
2. **Bir veya daha fazla tetikleyici.** Agent'i uyandıran olayların listesi - yeni bir yorum, oy veya rapor eşiğini aşan bir yorum, bir moderatör işlemi, bir kullanıcının sitedeki ilk yorumu ve diğerleri. Tam liste [Tetikleyici Olaylar Genel Bakışı](#triggers-overview) sayfasındadır.
3. **Bir araç izin listesi.** Agent'in neler yapmasına izin verildiği - bir yorum göndermek, oy vermek, sabitlemek, kilitlemek, spam olarak işaretlemek, bir kullanıcıyı yasaklamak, DM ile uyarmak, bir rozet vermek, e-posta göndermek, paylaşılan bir belleği kaydetmek ve aramak. Tam liste [İzin Verilen Araç Çağrıları Genel Bakışı](#tools-overview) sayfasındadır.

Bir tetikleyici çalıştığında, agent ne olduğunu tanımlayan bir bağlam mesajı (yorum, sayfa, isteğe bağlı konu/kullanıcı/sayfa bağlamı) alır ve başlangıç istemi ile topluluk yönergeleriniz gösterilir. Ardından eylemde bulunmak için araçları çağırır ve her çağrıda bir gerekçe ile bir güven puanı kaydeder.

### Agent'ler eşzamansız çalışır

Agent'ler **onları tetikleyen kullanıcı eylemini asla engellemez**. Bir okuyucu bir yorum gönderir, yorum kaydedilir ve konuya gösterilir, yanıt döndürülür ve ancak *ondan sonra* agent üzerinde çalışır — ya hemen ya da yapılandırılmış bir gecikmeden sonra (bkz. [Ertelenmiş Tetikleyiciler](#trigger-deferred-delay)). Agent'in yaptığı hiçbir şey kullanıcıya görünen deneyime gecikme eklemez.

### Neden kullanılır

- **Ölçekli moderasyon.** Açık spamleri işaretleyin ve tekrar eden ihlal yapanları sürekli kuyruğu izlemeye gerek kalmadan yasaklayın.
- **Yeni yorumcuları karşılayın.** İlk kez yorum yapanlara kendi sesinizle cevap verin.
- **En iyi içeriği öne çıkarın.** Oy eşiğini aştığında önemli üst düzey yorumları sabitleyin.
- **Yönergelerinizi tutarlı şekilde uygulayın.** Her sınırdaki yoruma aynı politika metnini uygulayın.
- **Uzun başlıkları özetleyin.** Çok sayfalı tartışmaların tarafsız özetlerini gönderin.

### Sizi kontrol altında tutanlar

- **Kuru Çalıştırma modu.** Her yeni agent **Dry Run** modunda gelir: tetikleyicileri işler, modeli çalıştırır ve ne yapacağını kaydeder, ancak gerçek bir işlem yapmaz. Bkz. [Kuru Çalıştırma Modu](#dry-run-mode).
- **Onaylar.** Herhangi bir işlem alt kümesi insan onayının arkasına alınabilir. Bkz. [Onay İş Akışı](#approval-workflow).
- **Agent başına ve hesap başına bütçeler.** Günlük ve aylık katı sınırlar. Bkz. [Bütçeler Genel Bakışı](#budgets-overview).
- **Araç izin listesi.** İzin verilmeyen araçlar modelin paletinden çıkarılır - agent gerçekten onları talep edemez. Bkz. [İzin Verilen Araç Çağrıları Genel Bakışı](#tools-overview).
- **Her işlemde denetim alanları.** Model bir gerekçe ve güven puanı içermelidir. Her ikisi de çalışma zaman çizelgesinde ve her onayda görünür. Bkz. [Çalışma Detay Görünümü](#run-detail-view).
- **AB DSA Madde 17.** AB bölgesinde tam otomatik yasaklamalar engellenir. Bkz. [AB DSA Madde 17 Uyum](#eu-dsa-compliance).
- **Verileriniz üzerinde eğitim yapılmaz.** FastComments, istemleriniz veya yorumlarınız üzerinde eğitim yapmayan sağlayıcıları kullanır.

### İnsan moderasyonunun yanında nasıl yer alırlar

Agent'ler ve insan moderatörler aynı yorum platformunu paylaşır: agent'ler aynı kanallar aracılığıyla işlem yapar (approve, spam, ban, badge, pin, lock, write) ve bu işlemler aynı [Yorum Kayıtları](/guide-moderation.html#comment-logs), aynı [Moderasyon Sayfası](/guide-moderation.html#moderate-comments-page) ve aynı bildirim akışlarında görünür. Agent'ler ve insanlar birbirlerinin işlerini görür ve birbirlerine tepki verebilir — moderatör işlemleri kendileri de geçerli agent tetikleyicileridir (bkz. [Tetikleyici: Moderatör Tarafından İncelenen Yorum](#trigger-moderator-reviewed) ve benzerleri).