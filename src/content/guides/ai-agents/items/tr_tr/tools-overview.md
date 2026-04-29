Bir ajanın **tools**'ı alabileceği eylemlerdir. Ajan düzenleme formunda bu ajanın kullanmasına izin verdiğiniz araçları işaretlediğiniz bir **Allowed tool calls** bölümü ve yürürlüğe girmeden önce bir insanın onaylaması gereken eylemleri işaretlediğiniz bir **Approvals** bölümü vardır.

Her araç için üç seviye vardır:

- **Disallowed** - ajan bunu göremez veya kullanamaz.
- **Allowed, no approval** - ajan bunu doğrudan kullanır. Çalıştırma geçmişine kaydedilir.
- **Allowed, with approval** - ajanın çağrısı insan incelemesi için sıraya alınır ve yalnızca bir insan onayladığında çalışır.

Disallowed araçlar sessizdir: ajan bunları isteyemez ve platform bunları doğrudan reddeder. Onayla sınırlandırılmış araçlar her zaman [approvals inbox](#approval-workflow) üzerinden gider.

### Her eylem için denetim kaydı

Ajanın yaptığı her eylem kısa bir gerekçe (nedenini açıklayan 1-2 cümle) ve bir güven skoru (0.0-1.0) ile kaydedilir. Her ikisi de [Run Detail View](#run-detail-view) ve her [approval](#approval-workflow) üzerinde görünür. Bellek araması tek salt okunur istisnadır: bir eylem olarak kaydedilmez ve izin listesine bakılmaksızın her zaman kullanılabilir.

### Araç referansı

#### Yorum gönderme

Ajanın kendi adıyla bir yorum göndermesine izin verir. Yorum, ajanın gösterim adı altında kamuya açık şekilde gösterilir. Karşılama ve özetleme ajanları tarafından kullanılır. Geri alınabilir - herhangi bir moderatör kötü bir yorumu kaldırabilir. Genellikle onay olmadan izin verilir; topluluğunuz her kamuya açık mesajın insan tarafından incelenmesini gerektiriyorsa bunu onaya bağlayın.

#### Bir yorumu düzenleme

Ajanın kapsam içi bir yorumun metnini yeniden yazmasına izin verir. Orijinal metin yorumun denetim kaydında korunur. Dar durumlar için ayırın - bir kullanıcının sızdırdığı kişisel tanımlayıcı bilgileri (PII) gizlemek ya da ajanın kendi önceki yanıtını düzeltmek gibi. Görüşleri yeniden yazmak veya üslubu yumuşatmak için kullanılmaz. **Onayı zorunlu kılmayı şiddetle düşünün.** Tam sayfa için [Edit comment](#tool-edit-comment) bölümüne bakın.

#### Yorumlara oy verme

Ajanın bir yoruma olumlu veya olumsuz oy vermesini sağlar. Oy, diğer tüm oylar gibi yorumun oy toplamına eklenir. Çoğu topluluk botların oy kullanmasını tercih etmez; hiçbir başlangıç şablonunda etkin değildir. İzin verirseniz, oylar geri alınabilir.

#### Yorumu sabitle / sabitlemeyi kaldır

Ajanın bir yorumu sayfanın üstüne sabitlemesine veya zaten sabitlenmiş bir yorumu serbest bırakmasına izin verir. Platform bir başlık başına tek sabitleme kuralını zorlamaz, bu yüzden bir sabitleme ajanına önce önceki sabitlenmiş yorumu kaldırması talimatı verilmelidir. Top Comment Pinner şablonu tarafından kullanılır. Geri alınabilir; genellikle onay olmadan izin verilir.

#### Yorumu kilitle / kilidini aç

Ajanın bir yorum altındaki daha fazla yanıtı engellemesine veya yanıtları geri getirmesine izin verir. Kilitli yorum görünür kalır. Kızışmış başlıklarda soğuma süreleri için kullanışlıdır; ertelemeli kilit açma ile eşleştirildiğinde etkilidir. Geri alınabilir ancak topluluğunuza görünür; yüksek riskli topluluklarda onaya bağlamayı düşünün.

#### Spam olarak işaretle / işaretini kaldır

Ajanın bir yorumu spam olarak işaretlemesine (okuyuculardan gizleyip spam sınıflandırıcısına besleyerek) veya bu bayrağı temizlemesine izin verir. Her moderasyon ajanı için temel araçtır. Geri alınabilir. Ajana güven oluştururken ilk haftalarda onaya bağlamayı şiddetle düşünün.

#### Bir yorumu onayla / onayını kaldır

Ajanın beklemede olan bir yorumu okuyuculara göstermesine veya zaten görünür olanı gizlemesine izin verir. Yeni yorumları moderatör incelemesi için tutan kiracılarda en faydalıdır. Görünür bir yorumun onayını kaldırmak yüksek risklidir - onaya bağlamayı düşünün.

#### Bir yorumu incelendi olarak işaretle

Bir kuyruk-durumu aracı: bir yorumu "bir moderatörün (veya ajanın) bunu incelediğini" olarak işaretler. Görünürlüğü değiştirmez. Düşük risklidir; nadiren onaya bağlanır.

#### Rozet verme

Ajanın kiracınızın rozet yapılandırmasından bir kullanıcıya rozet vermesine izin verir. Moderatör tarafından geri alınabilir. Nadiren onaya bağlanır. Ajanın rozet kimliğini bilmesi gerekir, bu yüzden ilgili kimlikleri [community guidelines](#community-guidelines) veya [initial prompt](#personality-prompt) içinde belirtin.

#### E-posta gönderme

Ajanın `noreply@fastcomments.com` adresinden seçtiği bir adrese düz metin e-posta göndermesine izin verir. İhtiyatlı kullanın - e-posta en yüksek sürtünmeli araçtır ve kötü e-postaları geri almak zordur. Onaya bağlamayı güçlü şekilde düşünün ve onay e-postalarını ajanın e-posta göndereceği gelen kutusunun sahibine yönlendirin.

#### Ajan belleğini kaydet / ara

Tetiklenen kullanıcı hakkında paylaşılan not havuzunu okuyan ve yazan iki eşleşmiş araçtır. Bellek kiracınızdaki tüm ajanlar arasında paylaşılır, bu nedenle bir triyaj ajanın notları bir moderatör ajanın kararlarını bilgilendirir. Arama salt okunurdur ve her zaman kullanılabilir; kaydetme nadiren onaya bağlanır. Tam tasarım için [Agent Memory System](#agent-memory-system) bölümüne bakın.

#### Bir kullanıcıyı uyar

Belirli bir yorum hakkında bir kullanıcıya özel bir DM uyarısı gönderir ve uyarıyı atomik olarak ajan belleğine kaydeder. Platformun yükseltme politikası bu araç etrafında kuruludur - önce uyarın, kullanıcı yeniden ihlal ederse yalnızca o zaman yasaklayın. `ban_user`'dan daha az sıklıkla onaya bağlanır, ancak bir ajanın yaşamının ilk haftalarında onaya bağlamayı düşünün. Tam sayfa için [Warn user](#tool-warn-user) bölümüne bakın.

#### Bir kullanıcıyı yasakla

Ajanın çağırabileceği en sonuç doğurucu araç. Bir kullanıcıyı sabit süreli olarak yasaklar; isteğe bağlı olarak gölge yasaklama, isteğe bağlı olarak IP yasaklama ve isteğe bağlı olarak kullanıcının tüm yorumlarını silme seçenekleri vardır. İki yıkıcı seçenek (IP, delete-all-comments) düzenleme formundaki ekstra onay tercihleri arkasında saklanır. Model parametreyi uydarsa bile platform etkinleştirmediğiniz değerlere izin vermez. [Ban user](#tool-ban-user) bölümüne bakın.

### Yasaklama aracının alt-seçenekleri

Ban aracı iki yıkıcı seçeneği açığa çıkarır - delete-all-comments ve ban-by-IP - bunlar, düzenleme formundaki **Ban options** bölümünden etkinleştirene kadar modelden tamamen gizlidir. Model parametreyi uydarsa bile platform, etkinleştirmediğiniz değerlere izin vermez. Bkz. [Ban user](#tool-ban-user).