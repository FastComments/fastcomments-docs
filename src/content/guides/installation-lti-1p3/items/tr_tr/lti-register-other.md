#### Sakai

LTI Advantage içeren sürümlerde Sakai, LTI 1.3 Dynamic Registration'ı destekler. Administration Workspace üzerinden:

1. Sakai yöneticisi olarak oturum açın ve **Administration Workspace**'i açın.
2. **External Tools** > **Install LTI 1.3 Tool** seçeneğini seçin.
3. FastComments kayıt URL'sini yapıştırın (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alın</a>) ve gönderin.
4. El sıkışma tamamlandığında aracı onaylayın.

Ara daha sonra **External Tools** altında görünür ve site yöneticileri tarafından sitelere eklenebilir.

#### Schoology

Schoology Enterprise örnekleri LTI 1.3'ü destekler ancak Dynamic Registration'ın kullanılabilirliği dağıtıma göre değişir. Schoology hesap yöneticinizle görüşün.

Dynamic Registration Schoology örneğinizde kullanılamıyorsa, entegrasyonu şu uç noktaları kullanarak manuel olarak yapılandırmanız gerekir:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Schoology size bir Client ID ve Deployment ID verdikten sonra, yapılandırmayı kiracınızda (tenant) kaydetmek için FastComments desteğiyle iletişime geçin.

#### Diğer LTI 1.3 Platformları

IMS LTI 1.3 Advantage spesifikasyonuna uyan herhangi bir LMS, aynı kayıt URL'siyle çalışmalıdır (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alın</a>). "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" veya benzeri şekilde etiketlenmiş bir ayar arayın.

Platformunuz yalnızca manuel LTI 1.3 kurulumunu destekliyorsa, yukarıdaki Schoology bölümünde listelenen dört uç noktayı kullanın ve tamamlamak için destekle iletişime geçin.