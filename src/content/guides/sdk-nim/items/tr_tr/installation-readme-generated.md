### Nimble Kullanımı

```bash
nimble install fastcomments
```

### Kaynağından Derleme

```bash
nimble build
```

### Kütüphane İçeriği

Bu kütüphane, API ile çalışmayı kolaylaştıran oluşturulmuş API istemcisini ve SSO yardımcı programlarını içerir.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Genel ve Güvenli API'ler

API istemcisi için üç API modülü vardır: `api_default`, `api_public` ve `api_moderation`. `api_default`, API anahtarınızı gerektiren metodları içerirken, `api_public` kimlik doğrulama gerektirmeden doğrudan bir tarayıcı/mobil cihaz vb. üzerinden yapılabilen API çağrılarını içerir. `api_moderation` modülü, moderatör kontrol paneli için metodları içerir.

`api_moderation` modülü, canlı ve hızlı denetleme API'lerinin kapsamlı bir paketini sunar. Her `api_moderation` metodu bir `sso` parametresi alır ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.