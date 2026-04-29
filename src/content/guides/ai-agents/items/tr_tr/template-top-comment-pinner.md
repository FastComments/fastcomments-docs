---
**Şablon Kimliği:** `top_comment_pinner`

Top Comment Pinner, oy eşiğini aşan üst düzey yorumları izler ve onları sabitler - aynı konuda daha önce sabitlenmiş olanı değiştirir.

### Yerleşik başlangıç istemi

[inline-code-attrs-start title = 'Top Comment Pinner Şablonu Başlangıç İstemi'; type='text' inline-code-attrs-end]
[inline-code-start]
You pin the best top-level comments on a thread. When a comment reaches the vote threshold, pin it if the content is substantive and non-promotional. Unpin any previously pinned comment on the same thread first. Do not pin replies, only top-level comments.
[inline-code-end]

"Yanıtları sabitleme" talimatı önemlidir: sabitleme işlemi konu başlıklarında çalışır, bu yüzden bir yanıtı sabitlemek nadiren kullanışlıdır. "Reklam amaçlı olmayan" filtresi, ajanın popüler link-spam bir yorumu güçlendirmesini engeller.

### Tetikleyiciler

- **Bir yorum oy eşiğini aşar** (`COMMENT_VOTE_THRESHOLD`, varsayılan oy eşiği: 10).

Tetik, yorumun net oyları (`up - down`) yapılandırılmış eşiğe ulaştığında tetiklenir. Bu sayıyı düzenleme formunda konu başlıklarınızın ne kadar aktif olduğuna göre ayarlayın - orta düzeyde aktif siteler için 10 makul bir varsayılandır.

### İzin verilen araçlar

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Sabitleme yıkıcı değildir - anında geri alınabilir - bu yüzden bu şablon genellikle onay gerektirmeden çalıştırılır.

### Canlıya almadan önce önerilen eklemeler

- **[Context Options](#context-options) içinde "Include parent comment and prior replies in the same thread" seçeneğini işaretleyin.** Konu bağlamı olmadan ajan, zaten sabitlenmiş bir yorumu açıp açamayacağını güvenilir şekilde söyleyemez.
- **Oy eşiğini sitenize göre ayarlayın.** Yoğun konularda 10 çok sık olabilir; sakin konularda 10 hiç olmayabilir.
- **Sadece belirli site bölümlerinde sabitlenmiş yorum istiyorsanız URL'e göre kapsam belirlemeyi düşünün** - örneğin haber konuları için, duyuru konuları için değil.

### Çift sabitleme hakkında not

Ajanın istemi önce sabitlemeyi kaldırmasını, sonra sabitlemesini söyler, ancak model bu adımı kaçırırsa platform kendi başına konu başlığı başına tek bir sabitliğe izin veren bir kural uygulamaz (birden fazla olabilir). Sitenizde çift sabitleme sorun oluşturuyorsa, `pin_comment`'i onay sürecine bağlayın ve her birini gözden geçirin - veya daha sıkı bir istem yazın.

---