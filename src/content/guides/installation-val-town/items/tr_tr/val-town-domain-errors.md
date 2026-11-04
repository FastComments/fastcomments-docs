`demo` kiracısından çıktıktan sonra, widget bir yetkilendirme hatasıyla yüklenmeyi reddedebilir. Bunun nedeni, FastComments'un hesabınızın o alan adında kullanılmasına izin verilmesi gerektiğini bilmemesidir.

[Sitenizi hesabınıza eklemek için buraya gidin.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town burada ikinci bir bakmaya değer, çünkü bir val birden fazla ana bilgisayar adında erişilebilir olabilir:

- Her HTTP val'in uzun bir varsayılan uç noktası vardır: `<org>--<id>.web.val.run`.
- Özel bir alt alan adı talep etmek `<name>.val.run` ekler.
- [Özel bir alan adı](https://docs.val.town/vals/http/custom-domains/) üçüncüsünü ekler.
- Branch'ler kendi URL'lerini alır.

Widget'ı gerçekten hangi ana bilgisayar adlarından sunuyorsanız onları ekleyin. Her şeyi ayarladıktan sonra bir alt alan adı talep ederseniz, onu da ekleyin, aksi takdirde widget eski URL'de çalışır ve yenide başarısız olur.