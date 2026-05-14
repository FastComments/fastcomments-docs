#### LTI 1.3 Yapılandırmasına Gidin

FastComments'e giriş yapın ve <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">LTI 1.3 Yapılandırma sayfanıza</a> gidin.

Hesabınızın henüz LTI erişimi yoksa "LTI not enabled for this account" mesajını görürsünüz - planınızda etkinleştirmek için destek ile iletişime geçin.

#### Bir Platform Seçin (İsteğe Bağlı)

**Generate a Dynamic Registration URL** altında, FastComments'e hangi LMS'e bağlandığınızı söylemek için **Platform** açılır menüsünü kullanın:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Diğer LTI 1.3 platformu

Ayrıca **Auto-detect** olarak bırakabilirsiniz. Platform, kayıt sırasında LMS'inizin openid-configuration'ından okunur; açılır liste yalnızca ortaya çıkan yapılandırma için görüntü etiketini belirler.

#### URL Oluştur

**Generate URL** öğesine tıklayın. FastComments tek kullanımlık bir kayıt belirteci oluşturur ve size şu görünüme sahip bir URL gösterir:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Kopyalayın. Bu URL:

- **Tek kullanımlık**tır - LMS'iniz başarılı bir şekilde çağırdığında belirteç tüketilir.
- Kullanılmazsa **30 dakika** sonra süresi dolar.
- Gizli tutulmalıdır - URL'e sahip olan herkes bu 30 dakika içinde kiracınıza (tenant) karşı araç kaydı yapabilir.

#### Mevcut Yapılandırmalar

Kayıt başarıyla tamamlandığında, yeni yapılandırma aynı sayfadaki **Existing Configurations** tablosunda Platform, Issuer, Client ID ve Status ile görünür. Kayıt iptali yapmanız gerekirse bu tablodan yapılandırmaları silebilirsiniz.