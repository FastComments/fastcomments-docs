Uyarı aracı, belirli bir yorum hakkında kullanıcıya özel bir DM uyarısı gönderir ve aynı zamanda uyarıyı paylaşılan [ajan belleği](#agent-memory-system) içinde kaydeder. İki yazma atomiktir - kullanıcı, kayıtta olmayan bir uyarı görmez.

### Neden var

Platformun tırmanma politikası **önce uyarı, kullanıcı tekrar suç işlerse yalnızca yasaklama**. Uyarı aracı bu politikayı uygulanabilir kılar: kullanıcıya düzelme şansı verir ve uyarı kaydı, gelecekte bir ajanın yasaklamayı düşünmeden önce bellekte aradığında bulacağı kayıttır.

Araç ayrıca yinelenmeleri de ortadan kaldırır: ajanın aynı kullanıcıya aynı yorum hakkında zaten bir uyarı verdiği durumda, ikinci uyarı hiçbir işlem yapmaz. Bu nedenle aynı yorum üzerinde dönen veya tekrar tetiklenen bir LLM kullanıcıyı birden çok uyarıyla spamleyemez.

### Uyarıda ne bulunur

Kullanıcıya DM olarak gösterilen kısa bir mesaj (1000 karakterle sınırlandırılmış). Etkili uyarılar şunlardır:

- **Spesifik** - "Bu toplulukta adı geçen kullanıcılara yönelik kişisel saldırılara izin verilmez" ifadesi "yorumunuz işaretlendi" ifadesinden daha iyidir.
- **Kısa** - en fazla birkaç cümle.
- **Eyleme geçirilebilir** - kullanıcıya neyi değiştirmesi gerektiğini söyleyin. "Lütfen yorumunuzu düzenleyip adı geçen kullanıcıyı kaldırın, aksi takdirde yorumunuz kaldırılacaktır."

Mesajı siz yazmazsınız; ajan yazar, [başlangıç istemine](#personality-prompt) ve [topluluk yönergelerine](#community-guidelines) dayanarak. Sizin göreviniz iyi uyarılar üreten bir istem yazmaktır.

### Ne zaman izin verilmeli

Herhangi bir moderasyon tarzı ajan için. Moderator şablonu bunu varsayılan olarak etkinleştirir.

### Onaylar

[Ban user](#tool-ban-user) aracına göre genellikle daha az sıklıkla onaya tabidir. Bir ajanın yaşamının ilk haftalarında kötü uyarıları çıkmadan önce tespit edebilmek için onaya tabi tutulması (gating) değerlidir, ancak ajan güvenilir çıktı üretmeye başladığında çoğu işletmeci bu kapıyı kaldırır.

### Ayrıca bakınız

- [Kullanıcıyı yasakla](#tool-ban-user) - tırmanmada bir sonraki adım.
- [Ajan Bellek Sistemi](#agent-memory-system) - uyarı kayıtlarının bulunduğu yer.