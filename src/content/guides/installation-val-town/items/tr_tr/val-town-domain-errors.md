Once you switch off the `demo` tenant, the widget may refuse to load with an authorization error. This is because FastComments doesn't know it's supposed to allow your account to be used on that domain.

[Hesabınıza sitenizi eklemek için buraya gidin.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town burada ikinci bir bakışa değer, çünkü bir val birden fazla ana bilgisayar adından erişilebilir:

- Her HTTP val, uzun bir varsayılan uç noktaya sahiptir, `<org>--<id>.web.val.run`.
- Özel bir alt alan adı talep etmek `<name>.val.run` ekler.
- Bir [özel alan adı](https://docs.val.town/vals/http/custom-domains/) üçüncüsünü ekler.
- Şubeler kendi URL'lerini alır.

Widget'ı hizmet verdiğiniz tüm ana bilgisayar adlarını ekleyin. Ayarları yaptıktan sonra bir alt alan adı talep ederseniz, onu da ekleyin; aksi takdirde widget eski URL'de çalışır ve yeni URL'de başarısız olur.