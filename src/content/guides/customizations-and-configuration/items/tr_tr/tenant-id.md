---
[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Örneğin, yorum widget'ının Tenant ID'si "demo" olarak kullanılabileceğini fark edebilirsiniz:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Bu yalnızca yorum widget'ını denemek ve oynamak içindir. Üretim ortamında Tenant ID'nizi şu şekilde geçirirsiniz:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Tenant ID'niz hesabınızdaki yorum widget'ına zaten uygulanmış <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">kod örneğinde</a> bulunur.

Tenant ID'nizi ayrıca [API kimlik bilgileri sayfasında](https://fastcomments.com/auth/my-account/api-secret) bulabilir ve API anahtarlarınızı yönetebilirsiniz.

Bundan sonra, FastComments'a giriş yaptıysanız, kod örnekleri gerçek Tenant ID'nizi kullanacaktır (https://fastcomments.com adresinde oturum açmışsanız).

---