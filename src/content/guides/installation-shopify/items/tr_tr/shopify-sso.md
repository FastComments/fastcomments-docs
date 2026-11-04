**FastComments** bloğu, Shopify müşterilerinizin ayrı bir FastComments hesabı oluşturmadan kendileri olarak yorum yapabilmeleri için tek oturum açma (single sign-on) desteği sunar.

### Nasıl çalışır

Mağazanıza giriş yapmış bir ziyaretçi, **FastComments** bloğunun bulunduğu bir sayfayı açtığında:

1. Blok, Shopify `customer` nesnesini algılar.
2. Müşterinin adını ve e-postasını, imzalı bir app proxy isteği aracılığıyla FastComments'e gönderir.
3. FastComments, `shopify-{customerId}` anahtarıyla bir kullanıcı oluşturur veya eşleştirir; böylece aynı müşteri, oturumlar ve yeniden kurulumlar arasında her zaman aynı FastComments kullanıcısına karşılık gelir.
4. Ziyaretçinin adı yorumlarında görünür. Tekrar giriş yapması istenmez.

Ziyaretçi mağazaya giriş yapmamışsa, blok anonim yorumlamaya (veya widget yapılandırmanıza bağlı olarak FastComments giriş akışına) geri döner.

### SSO'yu kapatma

SSO, her **FastComments** bloğu için varsayılan olarak açıktır. Belirli bir blokta kapatmak için:

1. Shopify tema düzenleyicisini açın.
2. Bloğu içeren şablonu açın ve bloğu seçmek için tıklayın.
3. **SSO** seçeneğinin işaretini kaldırın.
4. **Save** (Kaydet) düğmesine tıklayın.

Yorum yapan kişilerin sohbet için ayrı bir kimlik seçmesini istiyorsanız SSO'yu kapatın. Örneğin, personelin farklı bir görünen ad altında yorum yaptığı dahili bir topluluk sayfası gibi.

### FastComments ne alır

Her müşteri için gönderilen SSO yükü şunları içerir:

- Shopify müşteri kimliğinden türetilmiş bir kullanıcı kimliği (`shopify-{customerId}`).
- Müşterinin e-postası (kullanıcıyı tanımlamak için kullanılır; herkese açık olarak gösterilmez).
- Müşterinin görünen adı (yorum yazarı adı olarak kullanılır).

Sipariş geçmişi, ödeme veya adres verisi gönderilmez. Yük sunucu tarafında imzalanır; müşterinin tarayıcısı hiçbir zaman bir kimlik bilgisi görmez.

### Giriş ve çıkış bağlantıları

SSO açıkken, yorum widget'ının giriş ve çıkış bağlantıları, standart Shopify müşteri hesabı rotaları olan `/account/login` ve `/account/logout` adreslerine işaret eder. Yapılandırılacak bir şey yoktur. Bu bağlantılar, müşteri hesapları etkinleştirilmiş her mağazada çalışır.