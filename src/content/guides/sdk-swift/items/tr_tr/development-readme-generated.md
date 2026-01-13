### İstemciyi Yeniden Oluşturma

API istemcisini en son OpenAPI spesifikasyonundan yeniden oluşturmak için:

1. FastComments sunucusunun yerel olarak `http://localhost:3001` adresinde çalıştığından emin olun
2. Güncelleme betiğini çalıştırın:

```bash
./update.sh
```

Bu işlem:
- En son OpenAPI spesifikasyonunu indirir
- Swift istemci kodunu oluşturur (API dokümantasyonu client/docs içinde)
- Her şeyin çalıştığını doğrulamak için paketi derleyip kontrol eder