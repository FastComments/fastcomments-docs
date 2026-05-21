The **FastComments** bloğu tek oturum açma (SSO) destekler, böylece Shopify müşterileriniz ayrı bir FastComments hesabı oluşturmadan kendi kimlikleriyle yorum yapabilirler.

### Nasıl çalışır

When a visitor who is logged into your store opens a page with the **FastComments** block:

1. Blok Shopify `customer` nesnesini algılar.
2. Müşterinin adını ve e-postasını imzalı bir uygulama vekil isteği aracılığıyla FastComments'e gönderir.
3. FastComments, `shopify-{customerId}` anahtarıyla bir kullanıcı oluşturur veya eşler, böylece aynı müşteri oturumlar ve yeniden yüklemeler arasında her zaman aynı FastComments kullanıcısına karşılık gelir.
4. Ziyaretçinin adı yorumlarında görünür. Yeniden oturum açmaları istenmez.

If the visitor is not logged into the store, the block falls back to anonymous commenting (or the FastComments sign-in flow, depending on your widget configuration).

### SSO'yu kapatma

SSO, her **FastComments** bloğu için varsayılan olarak açıktır. Belirli bir blokta kapatmak için:

1. Shopify tema düzenleyicisini açın.
2. Bloğu içeren şablonu açın ve bloğu seçmek için üzerine tıklayın.
3. **SSO** işaretini kaldırın.
4. **Kaydet**'e tıklayın.

Yorum yapanların konuşma için ayrı bir kimlik seçmesini istiyorsanız SSO'yu kapatın. Örneğin, personelin farklı bir görüntüleme adıyla yorum yaptığı dahili bir topluluk sayfası.

### FastComments'in aldığı veriler

Her müşteri için gönderilen SSO yükü şunları içerir:

- Shopify müşteri kimliğinden türetilmiş bir kullanıcı kimliği (`shopify-{customerId}`).
- Müşterinin e-postası (kullanıcıyı tanımlamak için kullanılır; halka açık şekilde gösterilmez).
- Müşterinin görüntüleme adı (yorum yazarı adı olarak kullanılır).

Sipariş geçmişi, ödeme veya adres verileri gönderilmez. Yük sunucu tarafında imzalanır; müşterinin tarayıcısı asla bir kimlik bilgisi görmez.

### Giriş ve çıkış bağlantıları

SSO açık olduğunda, yorum widget'ının oturum açma ve oturum kapatma bağlantıları standart Shopify müşteri hesap yolları olan `/account/login` ve `/account/logout` adreslerine işaret eder. Yapılandırılacak bir şey yoktur. Bu bağlantılar müşteri hesapları etkinleştirilmiş herhangi bir mağaza için çalışır.