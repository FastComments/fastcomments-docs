From the [AI Ajanları sayfası](https://fastcomments.com/auth/my-account/ai-agents) şu iki yolla bir ajan oluşturabilirsiniz:

- **Bir şablondan.** **Browse templates** öğesine tıklayın ve dört yerleşik başlangıç ajanından birini seçin. Form önceden doldurulmuş olarak gelir ve ajanın durumu **Kuru Çalıştırma**'dır. Bkz. [Starter Templates](#starter-templates).
- **Sıfırdan.** **Create new agent** öğesine tıklayın. Form boş olarak açılır.

Her iki durumda da, aynı düzenleme formu daha sonra kaydettiğiniz ve düzenlediğiniz formdur. Bu sayfa formu baştan sona yürütür.

### Temeller

- **Internal name.** Yalnızca yönetici panolarında (çalıştırma geçmişi, analizler, denetim kayıtları) kullanılan kısa bir tanımlayıcı. Küçük harf ve alt çizgilerle kullanımı uygundur: `moderator`, `welcome_greeter`. Eğer bir şablonun internal name'i zaten alınmışsa, form otomatik olarak son ek ekler (`tos_enforcer_2`, vb.).
- **Display name.** Ajan bir yorum yayınladığında herkese gösterilen isim. Okuyucularınızın gördüğü isim budur.
- **Status.** Devre Dışı, Kuru Çalıştırma veya Etkin. Yeni ajanlar varsayılan olarak her zaman Kuru Çalıştırma olur. Bkz. [Status States](#status-states).

### Model

LLM'i seçin. Bkz. [Choosing a Model](#choosing-a-model).

### Bütçe

Hesap para biriminizde isteğe bağlı günlük ve aylık sınırlar ile bir **Uyarı eşikleri** onay listesi (varsayılan %80 ve %100). Bkz. [Budgets Overview](#budgets-overview) ve [Budget Alerts](#budget-alerts).

### Kişilik

**Initial prompt**, tonu, rolü ve karar kurallarını tanımlayan sistem istemidir. Düz metin, şablon sözdizimi yok. Bkz. [Personality and the Initial Prompt](#personality-prompt).

### Bağlam

Context alanı üç onay kutusu, bir yönergeler metin alanı ve kapsam girdilerine sahiptir:

- Aynı başlıktaki üst yorumu ve önceki yanıtları dahil et.
- Yorumcunun güven faktörünü, hesap yaşını, yasak geçmişini ve son yorumlarını dahil et.
- Sayfa başlığını, alt başlığını, açıklamayı ve meta etiketlerini dahil et.
- Her istemin başına eklenen isteğe bağlı bir **Topluluk yönergeleri** metin bloğu.
- **Belirli sayfalara kısıtlama** - URL desen izin listesi (her satıra bir). Boş bırakmak kiracı-genel anlamına gelir.
- **Belirli yerellere kısıtlama** - çift listeli seçim aracı ile yerel izin listesi. Boş bırakmak her yerel anlamına gelir.

Daha fazla bağlam daha iyi kararlar üretir ancak çalıştırma başına token maliyetini artırır. Bkz. [Context Options](#context-options), [Community Guidelines](#community-guidelines) ve [Scope: URL and Locale Filters](#scope-url-locale).

### Tetikleyiciler

Listeden en az bir olay seçin. vote-threshold ve flag-threshold tetikleyicileri için ayrıca eşik değerini belirlemeniz gerekir. İsteğe bağlı **Çalıştırmadan önce gecikme** alanı, bir tetikleyici tetiklendiğinde yürütmeyi erteler (oylar hâlâ yerleşirken flag eşikleri için kullanışlıdır). Bkz. [Trigger Events Overview](#triggers-overview) ve [Deferred Triggers](#trigger-deferred-delay).

### İzin verilen araç çağrıları

Tam araç paletini açmak için **Allow any tool calls** kutusunu işaretleyin. Aksi halde ajanın kullanmasına izin verilen belirli araçları işaretleyin - izin verilmeyen araçlar modelin paletinden budanır ve dağıtım zamanında reddedilir. **Ban options** alt bölümü, yıkıcı ban varyantlarını (delete-all-comments, ban-by-IP) açık bir şekilde onaylama gerektiren bir kapı arkasına alır. Bkz. [Allowed Tool Calls Overview](#tools-overview) ve [Tool: ban_user](#tool-ban-user).

### Onaylar

Ajanın bunları yürütmeden önce bir insan tarafından onaylanması gereken eylemleri işaretleyin. Onaylar yalnızca ajanın çağırmasına izin verilen araçlara uygulanır; izin verilmeyen araçlar doğrudan reddedilir. AB bölgesinde, ban_user Dijital Hizmetler Yasası Madde 17 tarafından kilitlenmiştir. Bkz. [Approval Workflow](#approval-workflow) ve [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Onay bildirimleri

Onaylar etkinleştirilmişse, kime e-posta gönderileceğini seçin:

- **Tüm yöneticiler ve moderatörler** - hesap sahipleri, süper yöneticiler ve yorum moderatörü yöneticileri.
- **Belirli kullanıcılar** - çift listeli seçim aracından el ile seçilenler.

Her değerlendiricinin bireysel teslim sıklığı (anında, saatlik özet, günlük özet) kendi profilinde ayarlanır. Bkz. [Approval Notifications](#approval-notifications).

### İstatistikler

Salt okunur. Toplam çalıştırma sayısı, son çalıştırma zaman damgası ve ajanın yazdığı en son yorumun kimliği (varsa).

### Kaydet

**Save agent** öğesine tıklayın. Sayfa ajan listesine geri yönlendirilir. Yeni ajanlar kuru çalıştırmada tetiklemeleri almaya derhal uygun hale gelir.

### Daha sonra düzenleme

Ajan listesi sayfasındaki her satır ajana özel eylemleri gösterir: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, ve **Delete**. Bir ajanı düzenlemek, önceden kaydedilmiş çalıştırmaları geriye dönük etkilemez - geçmiş korunur. Replay anlık görüntüleri ayrıca repleyin başlatıldığı noktada ajanın yapılandırmasını dondurur, böylece kaydedilmiş bir repleyin sonuçları istemi düzenledikten sonra bile tekrarlanabilir kalır.