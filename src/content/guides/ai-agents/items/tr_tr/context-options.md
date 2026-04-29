The **Context** bölümündeki düzenleme formu, ajanın her çalıştırmada ne kadar bilgi alacağını kontrol eder. Daha fazla context daha iyi kararlar üretir ancak çalıştırma başına token maliyetini artırır, bu yüzden ajanın gerçekten ihtiyaç duyduğu kadarını vermek istersiniz.

### Her zaman dahil edilenler

Tüm onay kutuları işaretsiz olsa bile, ajanın context mesajı şunları içerir:

- The **trigger event type** (ör. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- The **page URL and URL ID** (biliniyorsa).
- Çalıştırmayı tetikleyen **yorum** varsa — ID, yazar kullanıcı ID'si, yazar görüntü adı, yorum metni, oy sayıları, flag sayısı, spam/onaylandı/inceleme bayrakları, parent ID. Yazarın e-posta adresi **hiçbir zaman** LLM sağlayıcısına gönderilmez (PII minimizasyonu).
- `COMMENT_EDIT` tetikleyicileri için **önceki yorum metni** (böylece ajan önce/sonra karşılaştırması yapabilir).
- `COMMENT_VOTE_THRESHOLD` tetikleyicileri için **oy yönü**.
- **Tetikleyen kullanıcı ID'si** ve **rozet ID'si** (moderator rozet tetikleyicileri için).
- Ajanın rozet verebilmesine izin verildiğinde, ajanın uygun bir rozet seçebilmesi için tenant'ınızın **badge catalog** (isim, görüntü etiketi, açıklama).

Tüm güvensiz metinler - yorum gövdeleri, yazar isimleri, sayfa başlıkları, kılavuz belgesinin kendisi - context mesajında `<<<COMMENT_TEXT>>> ... <<<END>>>` gibi işaretlerle **çerçevelenir**. Platformun sistem istemi modele bu çerçevelerin içindeki talimatları asla takip etmemesini söyler. Bu platformun istem-enjeksiyon savunmasıdır; bunu isteminizde tekrarlamanıza gerek yoktur.

### Üç onay kutusu

#### Include parent comment and prior replies in the same thread

Ekler:
- The **parent comment** - ID, yazar, metin.
- **Sibling replies** - aynı ana yoruma ait, aynı dizideki önceki yanıtlar.

Kullanışlı olduğu durumlar: bir yoruma bağlam içinde yanıt veren herhangi bir ajan (karşılama yapanlar, dizi özetleyiciler, konuşmalarda yanıtları okuyan moderatörler).

Maliyet: küçük ila orta. Belirli bir dizide kaç kardeş yanıt olduğuyla sınırlıdır.

#### Include commenter's trust factor, account age, ban history, and recent comments

Ekler **AUTHOR_HISTORY** bloğunu:

- Kayıttan bu yana geçen **hesap yaşı (gün cinsinden)**.
- **Güven faktörü (0-100)** - bu sitedeki kullanıcının ne kadar güvenilir olduğunu özetleyen FastComments puanı. Moderasyon kılavuzunda [Spam Detection](/guide-moderation.html#spam-detection) sayfasına bakın.
- **Önceki ban sayısı.**
- **Bu sitedeki toplam yorum sayısı.**
- **Yinelenen içerik sayısı** - kullanıcının yakın zamanda aynı metni paylaşıp paylaşmadığı (anti-spam sinyali).
- **Aynı IP üzerinden hesaplar arası sinyal** - farklı hesaplar altında aynı IP'den yapılan yorum sayısı (alt-hesap sinyali). IP hash'i asla LLM'e gönderilmez.
- **Son yorumlar** - kullanıcının en fazla 5 adet en son yorumu, her biri 300 karaktere kadar kısaltılmış olarak, güvensiz metin şeklinde çerçevelenmiş.

Kullanışlı olduğu durumlar: herhangi bir moderasyon ajanı. Bu olmadan model yeni hesapları ve uzun süredir iyi niyetle davranan kullanıcıları aynı tavırla banlayabilir.

Maliyet: orta. Son yorumlar en fazla token ekler.

#### Include page title, subtitle, description, and meta tags

Ekler **PAGE_CONTEXT** bloğunu - başlık, alt başlık, açıklama ve FastComments'ın sayfa için yakaladığı meta etiketler.

Kullanışlı olduğu durumlar: sayfanın neyle ilgili olduğunu bilmenin çıktı kalitesini önemli ölçüde artırdığı karşılama yapanlar ve dizi özetleyiciler.

Maliyet: küçük.

### Community guidelines

Dördüncü alan, **Community guidelines**, her çalıştırmada user-role context mesajına dahil edilen serbest metin politika bloğudur; yorum gövdeleri ve diğer kullanıcı tarafından sağlanan içerikler ile aynı şekilde güvensiz metin olarak çerçevelenir. Ajan bunu politika metni olarak okur ancak platform bunu bir sistem talimatı olarak değerlendirmez. Ne koyacağınız konusunda [Topluluk Kuralları](#community-guidelines) bölümüne bakın.

### Bağlamı seçici olarak ekleme

Bu onay kutuları küresel değil, ajana göre uygulanır. Yaygın bir örüntü:

- Karşılama yapan: sayfa bağlamı **açık**, dizi bağlamı **kapalı**, kullanıcı geçmişi **kapalı**.
- Moderatör: dizi bağlamı **kapalı**, kullanıcı geçmişi **açık**, sayfa bağlamı **kapalı**.
- Dizi özetleyici: dizi bağlamı **açık**, sayfa bağlamı **açık**, kullanıcı geçmişi **kapalı**.

Bir ajanın gerçekte yaptığı çağrılarda doğru olması için ihtiyaç duyduğu en az bağlamı kullanmaya çalışın — fazladan bağlam her çalıştırmada token maliyeti getirir, ajan bunu kullanmasa bile.