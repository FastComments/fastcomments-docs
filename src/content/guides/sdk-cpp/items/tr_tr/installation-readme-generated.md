### Bağımlılıkları Yükleme

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Kaynağından Derleme

```bash
mkdir build
cd build
cmake ..
make
```

### Kurulum

```bash
sudo make install
```

### Kütüphane İçeriği

Bu kütüphane, API ile çalışmayı kolaylaştıran oluşturulmuş API istemcisini ve SSO yardımcı araçlarını içerir.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Genel ve Güvenli API'lar

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi` ve `ModerationApi`. `DefaultApi`, API anahtarınızı gerektiren yöntemleri içerir ve `PublicApi`, kimlik doğrulama olmadan doğrudan bir tarayıcı/mobil cihaz vb. üzerinden yapılabilen yöntemleri içerir. `ModerationApi`, canlı ve hızlı denetleme API'lerinin kapsamlı bir paketini sağlar. Her `ModerationApi` yöntemi bir `sso` parametresi alır ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.