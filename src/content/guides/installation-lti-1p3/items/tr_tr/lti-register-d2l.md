D2L Brightspace, LTI Advantage yönetici arayüzü aracılığıyla Dinamik Kayıt sunar. Yönetici erişimine ihtiyacınız olacak.

#### Kayıt Ekranını Açın

1. Brightspace örneğinize yönetici olarak giriş yapın.
2. **Admin Tools** > **Manage Extensibility** > **LTI Advantage** yolunu izleyin.
3. **Register Tool** öğesine tıklayın. (Doğrudan URL `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### URL'yi Yapıştırın

Bir kayıt formu göreceksiniz. Ana alan **Tool initiation registration endpoint**'tir (bazı Brightspace sürümleri bunu "Tool Initiation Registration URL" olarak etiketler).

FastComments kayıt URL'sini o alana yapıştırın. Diğer alanları boş bırakın - kayıt el sıkışması sırasında FastComments bunları otomatik doldurur.

**Register**'a tıklayın.

#### Aracı Onaylayın

Brightspace, FastComments ile iletişim kuran, anahtarları değiş tokuş eden ve bir onay ekranı gösteren bir açılır pencere açar. Kayıt tamamlandığında açılır pencere kendiliğinden kapanır.

Yeni araç LTI Advantage araç listenizde görünür. Varsayılan olarak Brightspace yeni araçları **disabled** olarak işaretler - kurslarınızın kullanabilmesi için anahtarı **enabled** konumuna getirin.

#### Bir Dağıtım Ekleyin

Brightspace'te LTI araçlarının kurslarda kullanılabilmesi için bir **deployment** gereklidir:

1. Yeni kaydedilmiş FastComments aracını açın.
2. **View Deployments** > **New Deployment** öğesine tıklayın.
3. Dağıtıma bir ad verin (örn. "FastComments - All Courses"), kullanılacağı organizasyon birimlerini seçin ve kaydedin.

Bu dağıtım aracılığıyla ilk başlatmadan sonra FastComments `deployment_id`'yi yapılandırma kaydına sabitler - aynı istemci altında farklı bir dağıtımdan sonraki başlatmalar, yeniden kayıt yapmadığınız sürece reddedilecektir.