SSO, veya tek oturum açma, sizin veya kullanıcılarınızın FastComments kullanmak için başka bir hesap oluşturmadan oturum açmasına olanak tanıyan bir dizi konvansiyondur.

Anonim yorum yapmaya izin vermediğinizi varsayarsak, FastComments ile yorum yapmak için bir hesap gereklidir. Bu kayıt sürecini çok kolay hale getiriyoruz - kullanıcı sadece yorum yaparken e-posta adresini bırakır.
Bununla birlikte, bunun bile bazı sitelerin kaçınmak istediği ekstra bir sürtünme olduğunu anlıyoruz.

Bu sürtünmeyi, tüm siteniz için yalnızca bir oturum açma akışı bulundurarak azaltabiliriz.

### Nasıl edinirim?
Tüm hesap türleri şu anda SSO erişimi alır. Ancak, SSO kullanıcılarının maksimum sayısı paketine bağlı olarak değişecektir. Diğer özelliklerde olduğu gibi, Pro planlar ve üstü doğrudan geliştirme desteği sağlar.

Seçenekleri karşılaştıralım, ardından her birinin ayrıntılarına girelim.

### Kullanıcı ve Yorum Taşımaları

Disqus gibi SSO olan bir platformdan taşınırken, zaten kullanıcılarınız ve onların yorumları olacaktır.

Yorumlar, API, Import UI'miz veya müşteri desteği aracılığıyla taşıma sırasında içe aktarılır. Import UI, hata işleme, avatar ve medya çıkarımı ve yüklemelerini ve toplu iş izleme sistemini içerdiği için, desteklediği platformdan taşıyorsanız tercih edilir.

Kullanıcılar, ilk kez yorum dizinlerini görüntülediklerinde otomatik olarak eklenir. Alternatif olarak, API aracılığıyla önceden eklenebilirler, ancak bu işlem çok fazla avantaja sahip değildir.

Yorumlar içe aktarılmışsa ve SSO kullanıcıları API aracılığıyla manuel olarak eklenmemişse, yorumlar kullanıcının herhangi bir yorum dizinini ilk kez görüntülediğinde o kullanıcı hesabı oluşturulduğunda otomatik olarak kullanıcının hesabına aktarılır. Daha sonra orijinal olarak yazdıkları yorumları yönetebilir, düzenleyebilir ve silebilirler.

Otomatik taşıma e-posta veya kullanıcı adı (username) aracılığıyla yapılır. Bazı platformlar Disqus gibi dışa aktarımda e-posta sağlamaz, bu yüzden bu durumda kullanıcı adına geri döneriz.
- Eşleşen bir kullanıcı adını ve SSO yükünde bir e-posta gönderdiğiniz sürece, bildirimler ve mentionların çalışması için e-postayı bireysel yorum nesnelerine ekleyeceğiz.

Yorumlarınızı ve kullanıcılarınızı aynı anda içe aktarmak isteniyorsa, kullanıcılar API aracılığıyla içe aktarıldıktan sonra yorumları ilgili kullanıcı hesaplarına taşımak için destek ile çalışın.

Özetle, taşıma için en kolay yol şu şekildedir:

1. Yorumları içe aktarın.
   1. `Manage Data -> Imports` içinde Import UI kullanılıyorsa avatarlar ve diğer medya otomatik olarak taşınır.
2. Secure veya Simple SSO'yu kurun.
3. İlk kez giriş yaptıklarında kullanıcı başına otomatik olarak taşınmasına izin verin.
   1. Kullanıcının 50k'dan az yorumu varsa bu genellikle sayfa yükleme süresine bir saniyeden daha az ekler.

### WordPress Kullanıcıları
Eğer <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress eklentimizi</a> kullanıyorsanız kod yazmanıza gerek yok! Sadece eklentinin Yönetici sayfasına gidin, SSO Ayarları'na tıklayın ve ardından Etkinleştir'e tıklayın.

Bu sizi API anahtarınızı oluşturacak, WordPress kurulumunuza gönderecek ve SSO'yu açacak tek düğmeli bir sihirbaza götürecektir. Bunu sizin için tek bir düğme tıklamasına yoğunlaştırdık.

Eklentiyi ilk kez yüklüyorsanız, SSO Ayarları düğmesini içeren yönetici sayfasını görmeden önce kurulum sürecini tamamlamanız gerektiğini unutmayın.

#### WordPress SSO - Moderatörler

FastComments WordPress eklentisiyle yorum yaparken moderatörlerinizin yanında "Moderator" rozeti görünmesi için,
şu anda onların ayrıca FastComments kontrol panelinde Moderatör olarak eklenmiş olmaları ve e-postalarının doğrulanmış olması gerektiğini unutmayın.

### Özel Entegrasyonlar

Özel entegrasyonlar için iki seçenek vardır.

### Birinci Seçenek - Secure SSO

Secure SSO ile FastComments, yorum yapanın, oy verenin ve yorumları okuyan kişinin sitenizde gerçek bir kullanıcı olduğunu bilir.

Geçerli bir yük oluşturduğunuz sürece, kullanıcı her zaman sorunsuz bir yorum deneyimine sahip olacaktır.

Secure SSO ile SSO yükü **sunucu tarafında** HMAC kimlik doğrulaması kullanılarak oluşturulur ve ardından **istemci** üzerindeki widget'a iletilir.

Secure SSO ile kullanıcının hesabı FastComments kullanıcı tabanının geri kalanından **tamamen ayrı**dır. Bu, iki ortağımız Company A ve Company B varsa, her birinin "Bob" kullanıcı adına sahip bir SSO kullanıcısı olabileceği anlamına gelir.

#### Gereksinimler
- Temel düzeyde backend geliştirme bilgisi.
- Gizli API anahtarlarıyla ilgili temel bilgi.
- API geliştirme veya sunucu tarafı render alma hakkında temel bilgi.

#### Artıları
- Güvenli.
- Sorunsuz yorum deneyimi.

#### Eksileri
- Backend geliştirme gerektirir.

#### Kullanıcı Verilerini Güncelleme

Secure SSO ile her seferinde sso kullanıcı yükünü geçirdiğinizde, kullanıcıyı en son bilgilerle güncelleyeceğiz. Örneğin, kullanıcının kullanıcı adı `X` ise ve SSO yükünde `Y` gönderirseniz, kullanıcı adı `Y` olacaktır.

Bu yaklaşımla değerleri kaldırmak istiyorsanız onları `null` (tanımsız değil `undefined`) olarak ayarlayın.

#### Secure SSO API

Ayrıca SSO kullanıcıları ile etkileşim için bir API sağlıyoruz. bkz [the docs](/guide-api.html#sso-user-structure).

Secure SSO kullanıldığında, kullanıcılar sayfa yüklemesi sırasında arka planda otomatik olarak oluşturulur. Kullanıcılarınızı toplu olarak içe aktarmanıza gerek yoktur.

### İkinci Seçenek - Simple SSO

Secure SSO'nun alternatifi, kullanıcı bilgilerini basitçe yorum widget'ına geçirmektir.

Simple SSO ile bir e-posta sağlamak zorunlu değildir, ancak bunun olmaması durumunda yorumları "Doğrulanmamış" olarak gösterilecektir.

<sup>Not!</sup> 2022 başlarından itibaren Simple SSO ile kullanıcı adlarının FastComments.com genelinde benzersiz olması gerekmez.

İdeal olarak, Simple SSO yalnızca backend erişimi sağlamayan bir platform üzerinde geliştirme yaparken seçilmelidir.

#### Gereksinimler
- Temel düzeyde istemci tarafı geliştirme bilgisi.
- En azından kullanıcının e-postasını bilmek.

#### Artıları
- Basit.
- Tüm etkinlikler yine de doğrulanır.
- Kullanıcı hiç kullanıcı adını veya e-postasını girmez.

#### Eksileri
- İstemci tarafı yükünün herhangi bir kullanıcı olabilecek şekilde hazırlanabileceği için Secure SSO'dan daha az güvenlidir.

#### Simple SSO API

Simple SSO akışıyla otomatik olarak oluşturulan kullanıcılar `SSOUser` nesneleri olarak saklanır. `SSOUser` API aracılığıyla erişilebilir ve yönetilebilirler. bkz [the docs](/guide-api.html#sso-user-structure).