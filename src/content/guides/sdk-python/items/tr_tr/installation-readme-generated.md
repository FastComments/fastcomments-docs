### GitHub'dan Kurulum

Bir sürüm etiketi üzerinden doğrudan kurun (önerilir, tamamen tekrarlanabilir):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Yapıların deterministik olmasını sağlamak için bir dal yerine etiketi sabitleyin. Aynı biçim `requirements.txt` içinde de çalışır:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Her etiketli [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) ayrıca doğrudan bir ikili artefakt kurmayı tercih ederseniz eklenmiş bir wheel dosyasına sahiptir.

### Kütüphane İçeriği

Bu kütüphane iki modül içerir: oluşturulan API istemcisi ve API ile çalışmayı, SSO desteği dahil, kolaylaştıran el yazısı yardımcı programları içeren temel Python kütüphanesi.

- [API İstemci Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Temel Kütüphane Belgeleri, SSO Örnekleri Dahil](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Genel vs Güvenli API'ler

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerir; `PublicApi` ise kimlik doğrulama gerektirmeden doğrudan bir tarayıcı/mobil cihaz vb. üzerinden yapılabilen yöntemleri içerir. `ModerationApi`, canlı ve hızlı denetleme API'lerinin kapsamlı bir paketini sağlar. Her `ModerationApi` yöntemi bir `sso` parametresi alır ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.