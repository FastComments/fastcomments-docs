Add this line to your application's Gemfile:

```ruby
gem 'fastcomments'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install fastcomments
```

### Kütüphane İçeriği

Bu kütüphane, API ile çalışmayı kolaylaştıran oluşturulmuş API istemcisi ve SSO yardımcı programlarını içerir.

- [API İstemci Kütüphane Belgeleri](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Genel ve Güvenli API'ler

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerirken, `PublicApi` kimlik doğrulama gerektirmeden doğrudan bir tarayıcı/mobil cihaz vb. üzerinden yapılabilecek API çağrılarını içerir. `ModerationApi` ise moderatör kontrol panelini güçlendiren yöntemleri barındırır.

`ModerationApi`, canlı ve hızlı moderasyon API'lerinden oluşan kapsamlı bir set sunar. Her `ModerationApi` yöntemi bir `sso` parametresi kabul eder ve SSO ya da bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.