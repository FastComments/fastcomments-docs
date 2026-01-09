### Nimble Kullanımı

```bash
nimble install fastcomments
```

### Kaynaktan Derleme

```bash
nimble build
```

### Kütüphane İçeriği

Bu kütüphane, API ile çalışmayı kolaylaştırmak için oluşturulmuş API istemcisini ve SSO yardımcı programlarını içerir.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Genel ve Güvenli API'ler

API istemcisi için iki API modülü vardır, `api_default` ve `api_public`. `api_default`, API anahtarınızı gerektiren yöntemleri içerir; `api_public` ise tarayıcı/taşınabilir cihaz vb. üzerinden doğrudan kimlik doğrulama olmadan yapılabilecek API çağrılarını içerir.