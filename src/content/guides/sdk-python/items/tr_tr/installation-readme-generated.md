### GitHub'dan Yükleyin

Doğrudan bir sürüm etiketiyle kurun (önerilir, tamamen tekrarlanabilir):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Etiketi bir dal yerine sabitleyerek yapıların belirlenebilir olmasını sağlayın. Aynı biçim `requirements.txt` içinde de çalışır:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Etiketli her [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) ayrıca doğrudan bir ikili artefakt kurmayı tercih ederseniz eklenmiş bir wheel içerir.

### Kütüphane İçeriği

Bu kütüphane iki modül içerir: oluşturulmuş API istemcisi ve API ile çalışmayı, SSO desteği dahil, kolaylaştıran elle yazılmış yardımcı programları içeren temel Python kütüphanesi.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Genel vs Güvenli API'ler

API istemcisi için üç sınıf vardır, `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerir ve `PublicApi`, kimlik doğrulama olmadan bir tarayıcı/mobil cihaz/benzeri üzerinden doğrudan yapılabilen yöntemleri içerir. `ModerationApi`, canlı ve hızlı denetim API'lerinin kapsamlı bir paketini sağlar. Her `ModerationApi` yöntemi bir `sso` parametresi kabul eder ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.