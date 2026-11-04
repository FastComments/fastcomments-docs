**Şablon Kimliği:** `top_comment_pinner`

Top Comment Pinner, oy eşiğini aşan üst düzey yorumları izler ve bunları sabitler - aynı konuda daha önce sabitlenmiş olanın yerine koyar.

Yerleşik istem (prompt), ajanı yanıtları atlaması (sabitleme konu bazında çalışır, bu yüzden bir yanıtı sabitlemek nadiren faydalıdır) ve tanıtım içeriklerini filtrelemesi (böylece ajan popüler bağlantı-spam'lerini yükseltmez) konusunda yönlendirir.

### Tetikleyiciler

- **Bir yorum oy eşiğini aşar** (`COMMENT_VOTE_THRESHOLD`, varsayılan oy eşiği: 10).

Tetikleyici, yorumun net oyu (`up - down`) yapılandırılan eşiğe ulaştığında devreye girer. Sayıyı, konularınızın ne kadar aktif olduğuna göre düzenleme formunda ayarlayın - orta düzeyde aktif siteler için 10 makul bir varsaylandır.

### İzin verilen araçlar

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Sabitleme yıkıcı değildir - anında geri alınabilir - bu yüzden bu şablon genellikle onay gerektirmeden çalışır.

### Yayına almadan önce önerilen eklemeler

- [Context Options](#context-options) bölümünde **"Include parent comment and prior replies in the same thread"** seçeneğini işaretleyin. Konu bağlamı olmadan ajan, sabitlenmiş bir yorumun olup olmadığını ve sabitlenmesi gerekeni güvenilir şekilde anlayamaz.
- **Oy eşiğini sitenize göre ayarlayın.** Yoğun konularda 10 çok sık gerçekleşir; sakin konularda 10 hiç gerçekleşmeyebilir.
- **URL'e göre kapsamlandırmayı düşünün** - yalnızca sitenizin belirli bölümlerinde sabitlenmiş yorumlar istiyorsanız; örneğin haber konularında, ama duyuru konularında değil.

### Yinelenen sabitleme hakkında not

Ajanın istemi, önce sabitlemeyi kaldırmasını sonra sabitlemesini söyler, ancak model bu adımı kaçırırsa platformun kendisi konu başına tek sabitlenmiş yorum kuralını zorunlu tutmaz (birden fazla olabilir). Sitenizde yinelenen sabitleme bir sorunsa, `pin_comment` aracını onay arkasına alın ve her birini gözden geçirin - ya da daha sıkı bir istem yazın.