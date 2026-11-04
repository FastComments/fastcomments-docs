**Şablon Kimliği:** `gaslight_detector`

Gaslight Dedektörü, bir sohbetin ortasında geçmişi yeniden yazan yorum düzenlemelerini izler - yani yazar, yanıtlar yazıldıktan sonra önceki bir yorumun anlamını değiştirdiğinde, alttaki yanıtların bağlam dışında veya yanlış görünmesine neden olan türden düzenlemeler. Ajan, bir düzenlemenin bu çizgiyi aştığına karar verdiğinde, orijinal metni geri yükler ve yazarına nedenini açıklayan bir DM gönderir.

Bu, kullanıcı içeriğini değiştirdiği için daha yüksek riskli bir şablondur. Salt okunur bir şablona göre daha uzun süre [dry-run](#dry-run-mode) modunda çalıştırın ve modelin trafiğiniz üzerindeki yargısına güvenene kadar `edit_comment` aracını [onay](#approval-workflow) arkasında tutun.

### Tetikleyiciler

- **Yorum düzenlendi** (`COMMENT_EDIT`) - ajan yeni ve önceki metni karşılaştırır ve düzenlemenin zaten var olan yanıtları bozup bozmadığına karar verir.

Düzenleme anındaki önceki yorum metni ve yanıt sayısı dahil tam payload için [Trigger: Comment Edited](#trigger-comment-edit) bölümüne bakın.

### İzin verilen araçlar

- [`edit_comment`](#tool-edit-comment) - düzenleme gaslighting olarak değerlendirildiğinde orijinal metni geri yüklemek için kullanılır.
- [`warn_user`](#tool-warn-user) - kullanıcının bir sonraki ziyaretinde göreceği yumuşak bir uyarı gönderir.
- [`send_dm`](#tools-overview) - açıklama kanalı; kullanıcı, düzenlemesinin neden geri alındığını açıklayan doğrudan bir mesaj alır.

Banlayamaz, spam işaretleyemez, oy veremez veya yeni yorum gönderemez - yüzey bilinçli olarak dardır.

### Canlıya alınmadan önce önerilen eklemeler

- **`edit_comment` aracını [onay](#approval-workflow) arkasına alın.** Bir yorumu geri almak, yazar için ve düzenlenmiş sürümü gören herkes için görünürdür, bu yüzden bir yanlış pozitif utanç vericidir. Dry-run, ajanın tutarlı olduğunu gösterene kadar onayları açık tutun.
- **Prompt'u sitenizde gaslighting'in ne sayıldığıyla sıkılaştırın.** Varsayılan prompt bilinçli olarak kısadır. Modele somut örnekler verin - "evet/hayır iddiasını tersine çevirmek", "yanıtların atıf yaptığı bir sayıyı silmek", "yanıtlar gönderildikten sonra düşmanca bir cümle eklemek" - ve yazım düzeltmeleri, biçimlendirme temizliği veya kaynak ekleme gibi açıkça örnek olmayan durumları belirtin.
- **Tetikleyici bağlamındaki yanıt sayısını kullanın.** Sıfır yanıtlı yorumlara yapılan düzenlemeler bir sohbeti bozamaz; prompt, modelin bunları atlamasını söylemelidir.
- **[Context Options](#context-options) bölümünde "Include commenter's trust factor, account age, ban history, and recent comments" seçeneğini işaretleyin.** Model, uzun süredir iyi niyetli bir hesap görebildiğinde çok daha az agresif olur.
- **Prompt'ta kısa bir düzenleme tolerans penceresi düşünün.** İlk 30-60 saniye içindeki birçok düzenleme yazım düzeltmesidir; modele bu kadar hızlı düzenlemeleri yok saymasını söyleyin.

### Önerilen dry-run süresi

Enabled'a geçmeden önce en az iki hafta gerçek trafik boyunca [dry-run](#dry-run-mode) modunda çalıştırın ve bu süre boyunca işaretlenen her düzenlemeyi inceleyin. Canlıya almadan önce son 30 günlük düzenlemeleri ajana karşı yeniden oynatmak için [Test Runs (Replays)](#test-runs-replays) özelliğini kullanın.