**Template ID:** `gaslight_detector`

Gaslight Dedektörü, bir konuşmanın ortasında geçmişi yeniden yazan yorum düzenlemelerini izler - yazarın daha önceki bir yorumun anlamını yanıtlar yazıldıktan sonra değiştirip, sonraki yanıtların bağlam dışı veya yanlış görünmesine yol açtığı türden düzenlemeler. Ajan düzenlemenin bu çizgiyi aştığına karar verirse, orijinal metni geri yükler ve yazara nedenini açıklayan bir DM gönderir.

Bu, kullanıcı içeriğini değiştirdiği için daha yüksek riskli bir şablondur. Okuma-yazma olmayan bir şablondan daha uzun süre [dry-run](#dry-run-mode) modunda çalıştırın ve `edit_comment` eylemini [onay](#approval-workflow) arkasına alıncaya kadar kapatın; modelin trafik üzerinde verdiği kararı güvenene kadar.

### Triggers

- **Comment edited** (`COMMENT_EDIT`) - ajan yeni ve önceki metni karşılaştırır ve düzenlemenin zaten var olan yanıtları çarpıtip çarpıtmadığına karar verir.

Bkz. [Trigger: Comment Edited](#trigger-comment-edit) tam yük için; önceki yorum metni ve düzenleme zamanındaki yanıt sayısı dahil.

### Allowed tools

- [`edit_comment`](#tool-edit-comment) - düzenleme gaslighting olarak değerlendirildiğinde orijinal metni geri yüklemek için kullanılır.
- [`warn_user`](#tool-warn-user) - kullanıcının sonraki ziyaretinde göreceği hafif bir uyarı verir.
- [`send_dm`](#tools-overview) - açıklama kanalı; kullanıcıya düzenlemenin neden geri alındığını anlatan bir direkt mesaj gönderir.

Banlama, spam işaretleme, oy kullanma veya yeni yorum gönderme işlemlerini yapamaz - yüzey kasıtlı olarak dar tutulmuştur.

### Recommended additions before going live

- **Gate `edit_comment` behind [approval](#approval-workflow).** Bir yorumu geri almak, hem yazara hem de düzenlenmiş versiyonunu görmüş herkese görünür olduğundan yanlış pozitif utandırıcı olabilir. Ajan tutarlı olana kadar onayları açık tutun.
- **Tighten the prompt with what counts as gaslighting on your site.** Varsayılan istem kasıtlı olarak kısa tutulmuştur. Modele somut örnekler verin - "evet/hayır iddiasını tersine çevirme", "yanıtların alıntıladığı bir sayıyı silme", "yanıtlar yayınlandıktan sonra düşmanca bir cümle ekleme" - ve yazım düzeltmeleri, biçimlendirme temizliği veya kaynak ekleme gibi açık olmayan örnekleri de belirtin.
- **Use the reply count from the trigger context.** Sıfır yanıtlı yorumlara yapılan düzenlemeler bir konuşmayı çarpıtamaz; istem modele bu tür düzenlemeleri atlamasını söylemelidir.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** in [Context Options](#context-options). Model uzun süredir iyi niyetle kullanılan bir hesabı gördüğünde çok daha az agresif olur.
- **Consider a short edit-grace window in the prompt.** İlk 30–60 saniye içindeki birçok düzenleme yazım düzeltmesidir; modele bu kadar kısa süredeki düzenlemeleri yok saymasını belirtin.

### Recommended dry-run window

Gerçek trafiğin en az iki haftası boyunca [dry-run](#dry-run-mode) modunda çalıştırın ve Etkinleştirmeden önce bu süre zarfında işaretlenen her düzenlemeyi gözden geçirin. Canlıya geçmeden önce ajanı son 30 gündeki düzenlemelerle tekrar yürütmek için [Test Runs (Replays)](#test-runs-replays) kullanın.