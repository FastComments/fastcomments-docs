D2L Brightspace, LTI Advantage yönetici arayüzü üzerinden Dinamik Kayıt'ı (Dynamic Registration) sunar. Yönetici erişimine ihtiyacınız olacak.

#### Kayıt Ekranını Açın

1. Brightspace örneğinize yönetici olarak giriş yapın.
2. **Admin Tools** > **Manage Extensibility** > **LTI Advantage** adresine gidin.
3. **Register Tool**'a tıklayın. (Doğrudan URL `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### URL'yi Yapıştırın

Bir kayıt formu göreceksiniz. Ana alan **Tool initiation registration endpoint**'tir (bazı Brightspace sürümleri bunu "Tool Initiation Registration URL" olarak etiketler).

Bu alana FastComments kayıt URL'sini (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alın</a>) yapıştırın. Diğer alanları boş bırakın - kayıt el sıkışması sırasında FastComments tarafından otomatik doldurulacaktır.

**Register**'a tıklayın.

#### Aracı Onaylayın

Brightspace, FastComments ile iletişim kuran, anahtarları değiştiren ve bir onay ekranı gösteren bir açılır pencere (popup) açar. Kayıt tamamlandığında açılır pencere kendiliğinden kapanır.

Yeni araç LTI Advantage araç listenizde görünür. Varsayılan olarak Brightspace yeni araçları **devre dışı** olarak işaretler - kurslarınızın kullanabilmesi için anahtarı **etkin** konuma getirin.

#### Bir Dağıtım Ekleyin

Brightspace'ta, LTI araçlarının derslerde kullanılabilmesi için bir **deployment** gereklidir:

1. Yeni kaydedilmiş FastComments aracını açın.
2. **View Deployments** > **New Deployment**'a tıklayın.
3. Dağıtıma bir isim verin (örn. "FastComments - All Courses"), kullanılacağı organizasyon birimlerini seçin ve kaydedin.

Bu dağıtım üzerinden ilk başlatmadan sonra FastComments, `deployment_id`'yi yapılandırma kaydına sabitler - aynı istemci altındaki farklı bir dağıtımdan sonraki başlatmalar, yeniden kayıt yapmadığınız sürece reddedilecektir.

---