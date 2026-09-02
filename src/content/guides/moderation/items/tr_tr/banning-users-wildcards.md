Belirli e-posta sağlayıcılarını kullanarak, joker karakterlerle kullanıcıları yasaklamak mümkündür.

Örneğin, **@bademail.com** adresinden gelen tüm yorumların spam olduğunu fark ederseniz, yasaklı bir kullanıcı eklerken e-posta giriş alanına "*@bademail.com" yazarak tüm e-posta sağlayıcısını basitçe yasaklayabilirsiniz.

E-postadaki @ işaretinden önceki "*" karakterine dikkat edin.

### Subdomains

Bir alan adı yasağı, o alan adının tüm alt alan adlarını da kapsar. `*@bademail.com` yasaklaması, `someone@mail.bademail.com` ve `someone@eu.mail.bademail.com` adreslerini de yasaklar, bu yüzden her alt alan adı için ayrı bir yasak eklemenize gerek yoktur.

Yalnızca belirli bir alt alan adını yasaklamak istiyorsanız, bunun yerine o alt alan adını girin; örneğin `*@mail.bademail.com`. Bu yasak, `someone@bademail.com` adresini etkilemez.

### Banning a Domain From a Comment

Deseni kendiniz yazmak zorunda değilsiniz. Yorumları Yönet sayfasında bir yorumdan bir kullanıcıyı yasakladığınızda, yasaklama iletişim kutusunda "Ban All @domain Users" adlı bir onay kutusu bulunur; bu, yorumcunun e-posta alanı için aynı `*@domain` yasaklamasını oluşturur.

### Supported Patterns

Desteklenen tek joker karakter biçimi, tam ad kısmının yerine tek bir `*` koyup ardından `@` ve bir alan adı eklemektir. Diğer biçimler kaydetmeye çalıştığınızda reddedilir:

- `*@*.bademail.com` gerekli değildir, çünkü `*@bademail.com` zaten alt alan adlarını kapsar.
- `name*@bademail.com` ve `*bademail.com` desteklenmez.

---