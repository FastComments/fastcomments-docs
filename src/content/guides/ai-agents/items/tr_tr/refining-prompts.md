**İstemi İyileştir** bir temsilcinin [initial prompt](#personality-prompt)'ini, üzerinde anlaşamadığınız belirli kararlara yanıt olarak düzenleme iş akışıdır. Bu işlem [approvals inbox](#approval-workflow)'tan başlatılır.

### Ne zaman kullanılmalı

Aynı türde bir onayı defalarca reddettiğinizi fark ettiğinizda - "temsilci, hedef göstermeyen şiddetli dil kullanan kişileri banlamak istiyor" gibi - temsilcinin istemi bunu düzeltmek için kaldıraçtır. İstemi İyileştir rehberli bir şekilde:

1. Kötü kararı temsil eden belirli bir onayı seçmenize,
2. temsilcinin ne yaptığını ve nedenini tam bağlamıyla birlikte istemi düzenlemenize,
3. yeni istemi temsilciye kaydetmenize olanak tanır.

Sonuç, ileriye dönük olarak aynı kararı verme olasılığı düşük olan bir temsilcidir.

### Akışı başlatma

Onay gelen kutusundan `/auth/my-account/ai-agent-approvals`:

1. Bir **rejected** onayı açın. Rota REJECTED dışındaki her şeyi sertçe reddeder - pending ve execution-failed onaylar uygun değildir.
2. **Refine prompt**'a tıklayın.

`/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt` adresinde prompt-refine UI'sine ulaşırsınız.

### Sayfada ne gösterilir

- **Onay** - reddedilen karar için temsilcinin `toolName` ve `justification` (tam LLM dökümü burada gösterilmez).
- **Mevcut istem** - temsilcinin kaydedilmiş [initial prompt](#personality-prompt)'i.
- **Bir geri bildirim girişi** - değiştirilmesi gerekenleri tanımlayan **geri bildirim** yazarsınız (en fazla 2000 karakter). Ardından LLM, geri bildiriminizden önerilen yeni istemi üretir.
- **Birleştirilmiş satır içi fark (unified inline diff)** - mevcut ile önerilen istem arasındaki tek bir satır içi fark (kaldırılanlar için kırmızı, eklenenler için yeşil).

Onay bağlamı, düzenleme yaparken "düzeltmekte olduğum vaka"ya başvurmayı sürdürebilmeniz için üstte sabit tutulur.

### Kaydet

Kaydetme, temsilcinin `initialPrompt` alanını günceller. Geçmiş çalıştırmalar (ve geçmiş onaylar) geriye dönük olarak yeniden çalıştırılmaz - yeni istem yalnızca gelecekteki tetiklemeleri etkiler. Yeni istemin sorunu düzeltip düzeltmediğini doğrulamak isterseniz, son 7 gün içinde bir [test run / replay](#test-runs-replays) çalıştırın ve yeni istemin reddedilen onayı yine üretip üretmeyeceğine bakın.

### Akışın yapmadıkları

- **Community guidelines** alanını düzenlemez - bu alanın kendi düzenleyicisi ana temsilci düzenleme formundadır.
- **Triggers**, **allowed tools** veya **approval gating** öğelerini düzenlemez - bunlar ana düzenleme formunda kalır.
- İstemi sürümlemez ve geri alma (rollback) sağlamaz. Önceki istem ayrı bir geçmiş koleksiyonunda depolanmaz. Geri almak gerekirse, düzenlemeden önce mevcut istemi kendi takip sisteminize kopyalayın.

### İnce ayarı yeniden oynatma ile eşleştirme neden gerekli

İstem düzenleyip sonucu test etmemek inanç temellidir. Önerilen döngü:

1. Bir onayı reddedin.
2. İstemi iyileştirin.
3. Son 7 gün için bir [test run](#test-runs-replays) çalıştırın.
4. **Deltas** sekmesine bakın. Yeni istem, kötü kararı "yapardı"dan "yapmazdı"ya mı taşıdı? Kazanımları da istemeden mi dışarı attı?
5. Yineleyin.

İnce ayar + yeniden oynatma döngüsünü üç veya dört kez yapmak genellikle bir moderasyon temsilcisi için kararlı bir istem elde etmeye yeter.

### Doğrudan düzenleme alternatifi

Refine Prompt kullanmak zorunda değilsiniz - temsilciyi ana düzenleme formunda da doğrudan düzenleyebilirsiniz. Refine Prompt'ın tek avantajı, belirli başarısız vakayı sabitleyerek neyi düzelttiğinizi kaybetmemenizi sağlamaktır.