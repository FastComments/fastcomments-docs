Düzenleme formundaki **Context** bölümü, her çalıştırmada ajanın ne kadar bilgi alacağını kontrol eder. Daha fazla bağlam daha iyi kararlar üretir ancak çalıştırma başına token maliyetini artırır, bu nedenle yalnızca ajanın gerçekten ihtiyaç duyduğu şeyi istersiniz.

### Her zaman dahil olanlar

Tüm onay kutuları işaretsiz olsa bile, ajanın bağlam mesajı şunları içerir:

- **Tetikleme olay türü** (örn. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- **Sayfa URL'si ve URL ID'si** (biliniyorsa).
- Çalıştırmayı tetikleyen **yorum**, varsa - ID, yazar kullanıcı ID'si, yazar görünen adı, yorum metni, oy sayıları, bayrak sayısı, spam/onaylandı/incelendi bayrakları, üst ID. Yazarın e-postası LLM sağlayıcısına **asla** gönderilmez (PII minimizasyonu).
- `COMMENT_EDIT` tetiklemeleri için **önceki yorum metni** (ajan öncesi/sonrası karşılaştırma yapabilsin diye).
- `COMMENT_VOTE_THRESHOLD` tetiklemeleri için **oy yönü**.
- **Tetikleyen kullanıcı ID'si** ve **rozet ID'si** (moderatör rozeti tetiklemeleri için).
- Ajanın rozet vermesine izin veriliyorsa, kiracınızın (tenant) **rozet kataloğu** (ad, görünen etiket, açıklama) - böylece ajan, rozetleri prompt'ta tek tek açıklamanıza gerek kalmadan uygun olanı seçebilir.

Tüm güvenilmeyen metinler - yorum gövdeleri, yazar adları, sayfa başlıkları, yönergeler belgesinin kendisi - bağlam mesajında `<<<COMMENT_TEXT>>> ... <<<END>>>` gibi işaretlerle **çitle çevrilir**. Platformun sistem prompt'u, modele bu çitlerin içindeki talimatları asla takip etmemesini söyler. Bu, platformun prompt-enjeksiyonu savunmasıdır; bunu kendi prompt'unuzda tekrarlamanıza gerek yoktur.

### Üç onay kutusu

#### Aynı konudaki üst yorumu ve önceki yanıtları dahil et

Şunları ekler:
- **Üst yorum** - ID, yazar, metin.
- **Kardeş yanıtlar** - aynı konuda aynı üste yapılmış önceki yanıtlar.

Şunlar için faydalıdır: bir yoruma bağlam içinde yanıt veren herhangi bir ajan (karşılama ajanları, konu özetleyiciler, konuşmalardaki yanıtları okuyan moderatörler).

Maliyet: küçük ile orta arası. Belirli bir konudaki kardeş yanıt sayısıyla sınırlıdır.

#### Yorumcunun güven faktörünü, hesap yaşını, yasak geçmişini ve son yorumlarını dahil et

**AUTHOR_HISTORY** bloğunu ekler:

- **Kayıttan bu yana hesap yaşı (gün)**.
- **Güven faktörü (0-100)** - kullanıcının bu sitede ne kadar güvenilir olduğunu özetleyen FastComments puanı. Moderasyon rehberindeki [Spam Detection](/guide-moderation.html#spam-detection) sayfasına bakın.
- **Önceki yasak sayısı.**
- **Bu sitedeki toplam yorum sayısı.**
- **Yinelenen içerik sayısı** - kullanıcı yakın zamanda aynı metni gönderdiyse (anti-spam sinyali).
- **Aynı IP'li çapraz hesap sinyali** - aynı IP'den diğer hesaplar altında yapılan yorum sayısı (alternatif hesap sinyali). IP hash'inin kendisi LLM'e asla gönderilmez.
- **Son yorumlar** - kullanıcının en son 5 yorumu, her biri 300 karaktere kırpılmış, güvenilmeyen metin olarak çitlenmiş.

Şunlar için faydalıdır: herhangi bir moderasyon ajanı. Bu olmadan, model yeni hesapları ve uzun süredir iyi niyetli kullanıcıları aynı tutumla yasaklar.

Maliyet: orta. Son yorumlar en fazla token'ı ekler.

#### Sayfa başlığını, alt başlığını, açıklamasını ve meta etiketlerini dahil et

**PAGE_CONTEXT** bloğunu ekler - FastComments'un sayfa için yakaladığı başlık, alt başlık, açıklama ve varsa meta etiketleri.

Şunlar için faydalıdır: karşılama ajanları ve konu özetleyiciler; sayfanın ne hakkında olduğunu bilmek çıktı kalitesini önemli ölçüde artırır.

Maliyet: küçük.

### Topluluk yönergeleri

Dördüncü alan olan **Community guidelines**, her çalıştırmada kullanıcı rolü bağlam mesajına dahil edilen, yorum gövdeleri ve diğer kullanıcı tarafından sağlanan içeriklerin çitlendiği şekilde güvenilmeyen metin olarak çitlenmiş, serbest metinli bir politika bloğudur. Ajan bunu politika metni olarak okur ancak platform bunu sistem talimatı olarak değerlendirmez. İçine ne koyacağınız için [Community Guidelines](#community-guidelines) bölümüne bakın.

### Bağlamı seçici olarak ekleme

Bu onay kutuları ajan başına geçerlidir, global olarak değil. Yaygın bir kalıp:

- Karşılama ajanı: sayfa bağlamı **açık**, konu bağlamı **kapalı**, kullanıcı geçmişi **kapalı**.
- Moderatör: konu bağlamı **kapalı**, kullanıcı geçmişi **açık**, sayfa bağlamı **kapalı**.
- Konu özetleyici: konu bağlamı **açık**, sayfa bağlamı **açık**, kullanıcı geçmişi **kapalı**.

Ajanın gerçekten yaptığı çağrılarda doğru olması için gereken minimum bağlama ulaşın - fazladan bağlam, ajan kullanmasa bile her çalıştırmada token maliyetine neden olur.