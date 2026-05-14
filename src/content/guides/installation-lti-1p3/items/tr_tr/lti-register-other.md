#### Sakai

Sakai, LTI Advantage içeren sürümlerde LTI 1.3 Dinamik Kaydı destekler. Yönetim Çalışma Alanı'ndan:

1. Bir Sakai yöneticisi olarak oturum açın ve **Yönetim Çalışma Alanı**'nı açın.
2. **Harici Araçlar** > **LTI 1.3 Aracını Yükle**'yi seçin.
3. FastComments kayıt URL'sini yapıştırın ve gönderin.
4. El sıkışma tamamlandığında aracı onaylayın.

Aracı daha sonra **Harici Araçlar** altında görünecek ve site sorumluları tarafından sitelere eklenebilir.

#### Schoology

Schoology Enterprise örnekleri LTI 1.3'ü destekler ancak Dinamik Kayıt kullanılabilirliği dağıtıma göre değişir. Schoology hesap yöneticinizle kontrol edin.

Schoology örneğinizde Dinamik Kayıt kullanılamıyorsa, entegrasyonu el ile aşağıdaki uç noktaları kullanarak yapılandırmanız gerekir:

- **OIDC Giriş URL'si**: `https://fastcomments.com/lti/v1p3/login`
- **Hedef Bağlantı URL'si**: `https://fastcomments.com/lti/v1p3/launch`
- **Genel Anahtar Kümeleri URL'si (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Yönlendirme URL'leri**: `https://fastcomments.com/lti/v1p3/launch`

Schoology size bir Client ID ve Deployment ID verdikten sonra, konfigürasyonu kiracınıza kaydetmek için FastComments desteği ile iletişime geçin.

#### Diğer LTI 1.3 Platformları

IMS LTI 1.3 Advantage spesifikasyonunu izleyen herhangi bir LMS aynı kayıt URL'si ile çalışmalıdır. "Dinamik Kayıt", "Araç Kayıt URL'si", "Araç başlatma kayıt uç noktası" veya benzeri bir etiketli bir ayar arayın.

Platformunuz yalnızca manuel LTI 1.3 kurulumunu destekliyorsa, yukarıdaki Schoology bölümünde listelenen dört uç noktayı kullanın ve sonlandırmak için destek ile iletişime geçin.