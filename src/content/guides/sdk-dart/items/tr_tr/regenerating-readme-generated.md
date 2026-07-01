---
Üreteç, mevcut olduğunda yerel olarak çalışan bir FastComments sunucusundan (`http://localhost:3001/js/swagger.json`) spesifikasyonu alır, aksi takdirde taahhüt edilen `openapi.json` dosyasına geri döner.

```bash
python3 update.py
```

`node`/`npx` (`@openapitools/openapi-generator-cli` için) ve Java gerektirir.
---