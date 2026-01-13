FastComments, isteklerin sitenizden geldiğini görmek için hesabınızdaki istekleri doğrular. Bu yüzden FastComments'i hangi siteye veya sitelere yüklemek istediğinizi bilmemiz gerekir.

FastComments, doğrulamayı hem alan adı hem de alt alan adları aracılığıyla destekler.

Örneğin `https://example.com` sitesini ele alalım. Bu durumda, "`example.com`" alan adıdır. `example.com` hem `example.com`'u hem de `www.example.com`'u destekler. "www"ye "alt alan adı" diyeceğiz.

For Example:

- To allow only `blog.example.com`:
  - Add `blog.example.com` to your domains.
- To allow `www.example.com`, `somesite.example.com`, and `example.com`:
  - Add `example.com` to your domains.
  - This is billed as having **one domain** associated with your account.
- You can now add wildcard subdomains, for example *myname.vercel.app. 
  - This is billed as having **one domain** associated with your account.

Eğer bir bloglama platformu kullanıyor ve size bir alt alan adı verildiyse, hesabınıza **alt alan adı da dahil olmak üzere tam alan adını** eklemek istersiniz; örneğin: `cats.blogger.com`.

We can add domains to our account by visiting the `My Domains` page and clicking `Add a Domain` at the bottom:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; title='The My Domains Page' app-screenshot-end]

Deneme süresi boyunca, **alan adları bu alanlardan gelen istekler olduğunda hesabınıza otomatik olarak eklenir**. Ancak bu süreden sonra güvenlik nedeniyle açıkça eklenmeleri gerekir. Bu otomatik davranış gerçekleştiğinde bir e-posta almanız gerekir.

Yerel geliştirme için `localhost` eklemeniz gerekmez - varsayılan olarak izinlidir.

#### Via The API

Alan adları ayrıca [DomainConfigs API aracılığıyla](/guide-api.html#domain-config-structure) eklenip yapılandırılabilir.