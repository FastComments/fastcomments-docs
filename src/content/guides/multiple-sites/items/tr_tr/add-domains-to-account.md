FastComments, isteklerin hesabınıza ait olduğunu ve sitenizden geldiğini doğrular. Bu yüzden  
FastComments'u hangi siteye veya sitelere kurmak istediğinizi bilmemiz gerekir.

FastComments, kimlik doğrulamayı alan adı ve alt alan adları üzerinden destekler.

Şimdi `https://example.com` sitesini ele alalım. Bu durumda, "`example.com`" alan adıdır. `example.com` hem `example.com` hem de `www.example.com` adreslerini destekler. "www" kısmını "alt alan adı" olarak adlandıracağız.

Örneğin:

- Yalnızca `blog.example.com` adresine izin vermek için:
  - `blog.example.com` adresini alan adlarınıza ekleyin.
- `www.example.com`, `somesite.example.com` ve `example.com` adreslerine izin vermek için:
  - `example.com` adresini alan adlarınıza ekleyin.
  - Bu, hesabınızla ilişkili **bir alan adı** olarak faturalandırılır.
- Artık joker karakterli alt alan adları ekleyebilirsiniz, örneğin *myname.vercel.app*.  
  - Bu da **bir alan adı** olarak faturalandırılır.

Bir blog platformu kullanıyorsanız ve size bir alt alan adı verilmişse,  
hesabınıza **alt alan adı dahil tam alan adını** eklemek istersiniz, örneğin: `cats.blogger.com`.

Hesabımıza alan adlarını eklemek için `My Domains` sayfasını ziyaret edip alttaki `Add a Domain` düğmesine tıklayabilirsiniz:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Hesaptaki alan adlarını listeleyen My Domains sayfası, alt kısımda Add a Domain düğmesi'; title='My Domains Sayfası' app-screenshot-end]

Deneme süresi boyunca, **alan adları istekler bu alan adlarından geldiğinde otomatik olarak hesabınıza eklenir**. Ancak,  
bu süreden sonra güvenlik nedeniyle alan adları açıkça eklenmelidir. Bu otomatik davranış gerçekleştiğinde bir e-posta almanız gerekir.

Yerel geliştirme için `localhost` eklemeniz **gerekmez** – varsayılan olarak izin verilir.

#### API Üzerinden

Alan adları ayrıca [DomainConfigs API](/guide-api.html#domain-config-structure) üzerinden eklenip yapılandırılabilir.