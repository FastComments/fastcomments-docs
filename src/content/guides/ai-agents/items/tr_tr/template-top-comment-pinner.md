**Template ID:** `top_comment_pinner`

The Top Comment Pinner, oy eşiğini geçen en üst düzey yorumları izler ve bunları sabitler - aynı dizide daha önce sabitlenmiş olanın yerini alır.

Yerleşik istem, ajana yanıtları atlamasını (sabitleme dizilerde çalıştığından, bir yanıta sabitlemek nadiren yararlıdır) ve tanıtıcı içeriği filtrelemesini söyler (ajanın popüler bağlantı-spam'ini desteklememesi için).

### Triggers

- **A comment crosses a vote threshold** (`COMMENT_VOTE_THRESHOLD`, default vote threshold: 10).

Tetikleyici, yorumun net oyları (`up - down`) yapılandırılmış eşiğe ulaştığında çalışır. Düzenleme formundaki sayıyı, dizilerinizin ne kadar aktif olduğuna göre ayarlayın - 10, orta düzeyde aktif siteler için makul bir varsayılandır.

### Allowed tools

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Sabitleme yıkıcı değildir - anında geri alınabilir - bu yüzden bu şablon genellikle onay gerektirmeden çalışır.

### Recommended additions before going live

- **Tick "Include parent comment and prior replies in the same thread"** in [Context Options](#context-options). Diziyi bağlamı olmadan ajan, zaten sabitlenmiş bir yorum olup olmadığını güvenilir şekilde söyleyemez.
- **Adjust the vote threshold** sitenize göre ayarlayın. Yoğun dizilerde 10 çok sık gerçekleşir; sessiz dizilerde 10 hiç olmayabilir.
- **Consider scoping by URL** yalnızca sitenizin belirli bölümlerinde sabitlenmiş yorumlar istiyorsanız - örneğin haber dizileri, fakat duyuru dizileri değil.

### Note on duplicate pinning

Ajana öncelikle sabitlemeyi kaldırması gerektiği öğütlenir, fakat model o adımı kaçırırsa platform kendisi dizide bir tane-sabitli-kuralını uygulamaz (birden fazla olabilir). Eğer sitenizde yinelenen sabitleme bir sorun teşkil ediyorsa, `pin_comment`'i onay arkasına alın ve her birini inceleyin - veya daha sıkı bir istem yazın.