The **Context** bölümü düzenleme formunda ajanın her çalıştırmada ne kadar bilgi alacağını kontrol eder. Daha fazla context (bağlam) daha iyi kararlar üretir ama her çalıştırma başına token maliyetini artırır, bu yüzden ajanın gerçekten ihtiyaç duyduğu kadarını istersiniz.

### Her zaman dahil olanlar

Tüm onay kutuları işaretlenmemiş olsa bile, ajanın context mesajı şunları içerir:

- The **trigger event type** (ör. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- The **page URL and URL ID** (biliniyorsa).
- The **comment** that triggered the run, if there is one - ID, author user ID, author display name, comment text, vote counts, flag count, spam/approved/reviewed flags, parent ID. Yazarın e-postası **asla** LLM sağlayıcısına gönderilmez (PII minimizasyonu).
- The **previous comment text** for `COMMENT_EDIT` triggers (ajanın öncesi/sonrası karşılaştırabilmesi için).
- The **vote direction** for `COMMENT_VOTE_THRESHOLD` triggers.
- The **triggering user ID** and **badge ID** (moderator badge tetiklemeleri için).

Tüm güvenilmeyen metinler - yorum gövdeleri, yazar adları, sayfa başlıkları, yönergeler belgesinin kendisi - context mesajında `<<<COMMENT_TEXT>>> ... <<<END>>>` gibi işaretlerle **çerçevelenmiştir**. Platformun sistem istemi modele bu çerçevelerin içindeki talimatları asla uygulamamasını söyler. Bu platformun prompt-injection savunmasıdır; bunu kendi isteminizde tekrarlamanıza gerek yoktur.

### Üç onay kutusu

#### Ebeveyn yorumu ve aynı konu içindeki önceki yanıtları dahil et

Ekler:
- The **parent comment** - ID, author, text.
- **Sibling replies** - aynı ebeveyne ait aynı konu içindeki önceki yanıtlar.

Kullanışlı olduğu durumlar: bir yoruma bağlam içinde yanıt veren herhangi bir ajan (karşılama botları, konu özetleyiciler, konuşmalardaki yanıtları okuyan moderatörler).

Maliyet: küçük ile orta. Belirli bir konuda kaç kardeş yorum olduğuna bağlıdır.

#### Yazarın güven faktörünü, hesap yaşını, yasak geçmişini ve son yorumlarını dahil et

Ekler **AUTHOR_HISTORY** bloğunu:

- Kayıttan bu yana gün cinsinden **hesap yaşı**.
- **Güven faktörü (0-100)** - kullanıcının bu sitede ne kadar güvenilir olduğunu özetleyen FastComments puanı. Moderasyon kılavuzundaki [Spam Tespiti](/guide-moderation.html#spam-detection) sayfasına bakın.
- **Önceki yasak sayısı.**
- **Bu sitedeki toplam yorum sayısı.**
- **Yinelenen içerik sayısı** - kullanıcı yakın zamanda aynı metni paylaştıysa (spam karşıtı sinyal).
- **Aynı-IP hesaplar arası sinyali** - aynı IP'den diğer hesaplara yapılan yorum sayısı (alter hesap sinyali). IP hash'i kendisi asla LLM'ye gönderilmez.
- **Son yorumlar** - kullanıcının en fazla 5 en son yorumu, her biri 300 karaktere kısaltılmış, güvenilmeyen metin olarak çerçevelenmiş.

Kullanışlı olduğu durumlar: her türlü moderasyon ajanı. Bu olmadan model, yeni hesapları ve uzun zamandır iyi niyetle davranan kullanıcıları aynı tutumla yasaklar.

Maliyet: orta. Son yorumlar en çok token ekleyen öğelerdir.

#### Sayfa başlığını, alt başlığını, açıklamasını ve meta etiketlerini dahil et

Ekler **PAGE_CONTEXT** bloğunu - başlık, alt başlık, açıklama ve FastComments'in sayfa için yakaladığı herhangi bir meta etiket.

Kullanışlı olduğu durumlar: sayfanın ne hakkında olduğunu bilmenin çıktı kalitesini önemli ölçüde artırdığı karşılama botları ve konu özetleyiciler.

Maliyet: küçük.

### Topluluk kuralları

Dördüncü alan, **Community guidelines**, her çalıştırmada kullanıcı-rolü context mesajına dahil edilen serbest metin politika bloğudur; yorum gövdeleri ve diğer kullanıcı kaynaklı içerikler ile aynı şekilde güvenilmeyen metin olarak çerçevelenir. Ajan bunu politika metni olarak okur ancak platform bunu sistem talimatı olarak değerlendirmez. Ne koyacağınız hakkında bilgi için bakınız [Topluluk Kuralları](#community-guidelines).

### Bağlamı seçerek ekleme

Bu onay kutuları küresel değil, ajana özgü olarak uygulanır. Yaygın bir desen:

- Hoş geldin karşılayıcısı: sayfa bağlamı **açık**, konu bağlamı **kapalı**, kullanıcı geçmişi **kapalı**.
- Moderatör: konu bağlamı **kapalı**, kullanıcı geçmişi **açık**, sayfa bağlamı **kapalı**.
- Konu özetleyicisi: konu bağlamı **açık**, sayfa bağlamı **açık**, kullanıcı geçmişi **kapalı**.

Ajandanın yaptığı çağrılarda doğru olmak için ihtiyaç duyduğu minimum bağlamı kullanın — fazladan bağlam her çalıştırmada token maliyeti getirir, ajan onu kullanmasa bile.