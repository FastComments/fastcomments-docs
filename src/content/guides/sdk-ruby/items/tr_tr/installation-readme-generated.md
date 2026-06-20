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

Bu kütüphane, oluşturulmuş API istemcisini ve API ile çalışmayı kolaylaştırmak için SSO yardımcı araçlarını içerir.

- [API İstemci Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Açık ve Güvenli API'ler

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi` API anahtarınızı gerektiren yöntemleri içerir, `PublicApi` ise kimlik doğrulama olmadan doğrudan bir tarayıcı/taşınabilir cihaz vb. üzerinden yapılabilecek API çağrılarını içerir. `ModerationApi` moderatör panosunu sağlayan yöntemleri içerir.

`ModerationApi` yorum moderasyonunu (listeleme, sayma, arama, günlükler, dışa aktarma), moderasyon eylemlerini (kaldır/geri yükle, işaretle, inceleme/istenmeyen/onay durumunu ayarla, oylar, konuyu yeniden aç/kapat), yasakları (bir yorumdan yasaklama, geri alma, ön-yasak özetleri, yasak durumu/tercihleri, yasaklı kullanıcı sayıları) ve rozetler & güveni (rozet ver/kaldır, manuel rozetler, güven faktörünü al/ayarla, kullanıcı iç profili) kapsar. Her `ModerationApi` yöntemi bir `sso` parametresi kabul eder; böylece istek, SSO ile doğrulanmış bir moderatör adına yapılabilir.