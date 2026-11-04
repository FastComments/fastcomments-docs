Uygulamanızın Gemfile'ına şu satırı ekleyin:

```ruby
gem 'fastcomments'
```

Ardından şunu çalıştırın:

```bash
bundle install
```

Ya da kendiniz şu şekilde kurun:

```bash
gem install fastcomments
```

### Kütüphane İçeriği

Bu kütüphane, API ile çalışmayı kolaylaştırmak için üretilmiş API istemcisini ve SSO yardımcı araçlarını içerir.

- [API İstemci Kütüphanesi Dokümantasyonu](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Herkese Açık ve Güvenli API'ler

API istemcisinde üç sınıf bulunur: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren metotları içerir; `PublicApi`, kimlik doğrulama gerektirmeden doğrudan bir tarayıcıdan/mobil cihazdan/vb. yapılabilecek API çağrılarını içerir. `ModerationApi`, moderatör panelini çalıştıran metotları içerir.

`ModerationApi`, canlı ve hızlı moderasyon için kapsamlı bir API paketi sunar. Her `ModerationApi` metodu bir `sso` parametresi kabul eder ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.