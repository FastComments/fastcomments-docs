#### Canvas LTI Yapılandırmasına Gidin

FastComments hesabınıza giriş yapın ve <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Hesabım > Canvas LTI Yapılandırması</a> bölümüne gidin.

#### Yeni Bir LTI Yapılandırması Oluşturun

**Enable LTI** onay kutusunu işaretleyin. Yapılandırma alanları görünecektir:

- **Configuration Name** - bu yapılandırmayı tanımlamak için isteğe bağlı bir etiket (birden fazla Canvas örneği bağlarsanız faydalıdır).
- **Platform URL** - Canvas örneğinizin URL'si (ör. `https://yourschool.instructure.com`). Şu an için bunu boş bırakabilir ve Geliştirici Anahtarını oluşturduktan sonra doldurabilirsiniz.
- **Client ID** - şu an için bunu boş bırakın. Bunu Canvas'ta Geliştirici Anahtarı oluşturduktan sonra dolduracaksınız.
- **Deployment ID** - şu an için bunu boş bırakın.
- **Comment Style** - Comments, Collab Chat veya Both arasından seçim yapın. Ayrıntılar için [Commenting Styles](#canvas-lms-commenting-styles) bölümüne bakın.

Yapılandırmayı oluşturmak için **Add**'e tıklayın.

#### Yapılandırma URL'lerini Kopyalayın

Kaydettikten sonra üç URL görünecektir:

- **Configuration URL** - bunu Canvas'ta Geliştirici Anahtarı oluştururken yapıştıracaksınız.
- **OIDC Login URL** - Canvas tarafından LTI giriş akışı için kullanılır (Yapılandırma URL'si aracılığıyla otomatik olarak yapılandırılır).
- **Launch URL** - bir öğrenci FastComments'i açtığında Canvas'ın çağırdığı uç nokta (Yapılandırma URL'si aracılığıyla otomatik olarak yapılandırılır).

**Configuration URL**'yi kopyalayın. Bir sonraki adımda buna ihtiyacınız olacak.