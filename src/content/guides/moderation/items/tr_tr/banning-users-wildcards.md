Joker karakterler kullanarak belirli e-posta sağlayıcılarını kullanan kullanıcıları yasaklamak mümkündür.

Örneğin, **@bademail.com** adresinden gelen tüm yorumların spam olduğunu fark ederseniz, yasaklı kullanıcı eklerken e-posta giriş alanına "*@bademail.com" girerek o e-posta sağlayıcısının tamamını yasaklayabilirsiniz.

E-postadaki @ işaretinden önceki "*" karakterine dikkat edin.

### Alt Alan Adları

Bir alan adı yasağı, o alan adının tüm alt alan adlarını da kapsar. `*@bademail.com` adresini yasaklamak, `someone@mail.bademail.com` ve `someone@eu.mail.bademail.com` adreslerini de yasaklar; bu nedenle her alt alan adı için ayrı bir yasak eklemenize gerek yoktur.

Yalnızca belirli bir alt alan adını yasaklamak istiyorsanız, bunun yerine o alt alan adını girin, örneğin `*@mail.bademail.com`. Bu yasak, `someone@bademail.com` adresini etkilemez.

### Bir Yorumdan Alan Adı Yasaklama

Deseni kendiniz yazmak zorunda değilsiniz. Yorumları Denetle sayfasında bir yorumdan bir kullanıcıyı yasakladığınızda, yasaklama iletişim kutusunda, yorum yapan kişinin e-posta alan adı için aynı `*@domain` yasağını oluşturan bir "Ban All @domain Users" onay kutusu bulunur.

### Desteklenen Desenler

Desteklenen tek joker karakter biçimi, ad kısmının tamamının yerine geçen tek bir `*` ve ardından `@` ile bir alan adıdır. Diğer biçimler, kaydetmeye çalıştığınızda reddedilir:

- `*@*.bademail.com` gerekli değildir, çünkü `*@bademail.com` zaten alt alan adlarını kapsar.
- `name*@bademail.com` ve `*bademail.com` desteklenmemektedir.