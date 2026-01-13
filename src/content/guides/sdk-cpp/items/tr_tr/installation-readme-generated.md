---
### Bağımlılıkları Yükleme

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Kaynak Koddan Derleme

```bash
mkdir build
cd build
cmake ..
make
```

### Yükleme

```bash
sudo make install
```

### Kütüphane İçeriği

Bu kütüphane, oluşturulmuş API istemcisini ve API ile çalışmayı kolaylaştırmak için SSO yardımcı programlarını içerir.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Genel ve Güvenli API'ler

API istemcisi için iki sınıf vardır, `DefaultAPI` ve `PublicAPI`. `DefaultAPI`, API anahtarınızı gerektiren yöntemleri içerir, ve `PublicAPI` API çağrıları içerir
ki bunlar kimlik doğrulama olmadan doğrudan bir tarayıcı/taşınabilir cihaz/vb. üzerinden yapılabilir.
---