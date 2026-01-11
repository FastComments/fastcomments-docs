### Testleri Çalıştırma

```bash
# Set up environment variables
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Run tests
pytest tests/
```

### İstemciyi Yeniden Oluşturma

API istemcisini en son OpenAPI spesifikasyonundan yeniden oluşturmak için:

```bash
./update.sh
```

Aşağıdakileri yapacaktır:
1. Çalışan bir FastComments sunucusundan en son OpenAPI spesifikasyonunu indirir (veya yerel openapi.yaml dosyasını kullanır)
2. Python istemci kodunu üretir
3. Dizin yapısını düzleştirir
4. Gereksiz yapılandırma dosyalarını temizler