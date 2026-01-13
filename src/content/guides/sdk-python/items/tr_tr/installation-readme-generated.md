### PyPI

```bash
pip install fastcomments
```

### Kütüphane İçeriği

Bu kütüphane iki modül içerir: oluşturulmuş API istemcisi ve SSO desteği de dahil olmak üzere API ile çalışmayı kolaylaştırmak için elle yazılmış yardımcı araçları içeren çekirdek Python kütüphanesi.

- [API İstemci Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Çekirdek Kütüphane Belgeleri, SSO Örnekleri Dahil](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Açık vs Güvenli API'ler

API istemcisi için iki sınıf vardır: `DefaultApi` ve `PublicApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerir; `PublicApi` ise kimlik doğrulama olmadan bir tarayıcı/taşınabilir cihaz vb. üzerinden doğrudan yapılabilecek API çağrılarını içerir.
---