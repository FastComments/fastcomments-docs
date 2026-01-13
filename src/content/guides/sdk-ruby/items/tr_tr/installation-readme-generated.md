---
Uygulamanızın Gemfile dosyasına bu satırı ekleyin:

```ruby
gem 'fastcomments'
```

Ve ardından şu komutu çalıştırın:

```bash
bundle install
```

Veya kendiniz şu şekilde kurun:

```bash
gem install fastcomments
```

### Library Contents

Bu kütüphane, oluşturulmuş API istemcisini ve API ile çalışmayı kolaylaştırmak için SSO yardımcı programlarını içerir.

- [API İstemci Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

API istemcisi için iki sınıf vardır, `DefaultApi` ve `PublicApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerir, ve `PublicApi` ise tarayıcı/taşınabilir cihaz gibi ortamlardan kimlik doğrulama olmadan doğrudan yapılabilecek API çağrılarını içerir.
---