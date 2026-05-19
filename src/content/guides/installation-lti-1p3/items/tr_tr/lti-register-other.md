#### Sakai

Sakai, LTI Advantage içeren sürümlerde LTI 1.3 Dinamik Kaydı destekler. Yönetim Çalışma Alanı'ndan:

1. Bir Sakai yöneticisi olarak oturum açın ve **Yönetim Çalışma Alanı**'nı açın.
2. **Harici Araçlar** > **LTI 1.3 Aracı Yükle** seçeneklerini seçin.
3. FastComments kayıt URL'sini yapıştırın (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alabilirsiniz</a>) ve gönderin.
4. El sıkışma tamamlandığında aracı onaylayın.

Araç daha sonra **Harici Araçlar** altında görünür ve site yöneticileri tarafından sitelere eklenebilir.

#### Schoology

Schoology Enterprise örnekleri LTI 1.3'ü destekler, ancak Dinamik Kayıt kullanılabilirliği dağıtıma göre değişir. Schoology hesap yöneticinizle kontrol edin.

Schoology örneğinizde Dinamik Kayıt kullanılamıyorsa, entegrasyonu manuel olarak şu uç noktaları kullanarak yapılandırmanız gerekir:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Schoology size bir Client ID ve Deployment ID verdikten sonra, yapılandırmayı kiracınıza kaydetmek için FastComments destek ekibiyle iletişime geçin.

#### Other LTI 1.3 Platforms

IMS LTI 1.3 Advantage spesifikasyonunu takip eden herhangi bir LMS, aynı kayıt URL'si ile çalışmalıdır (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alabilirsiniz</a>). "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" veya benzeri etiketli bir ayar arayın.

Platformunuz yalnızca manuel LTI 1.3 kurulumu destekliyorsa, yukarıdaki Schoology bölümünde listelenen dört uç noktayı kullanın ve işlemi tamamlamak için destek ile iletişime geçin.