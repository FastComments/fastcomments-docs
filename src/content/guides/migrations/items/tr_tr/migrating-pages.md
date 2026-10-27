---
Adı Domain Migration aracı olsa da, tek tek sayfalardaki yorumları taşımak için de kullanılabilir.

Sadece eski ve yeni [urlId](/guide-customizations-and-configuration.html#url-id) değerlerini `from` ve `to` giriş alanlarına girin.

### API ile Sayfaları Taşıma

Birçok sayfayı bir kerede taşımak için [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post) adresini kullanın. İstek başına en fazla 100 `from` ve `to` çifti alır ve bunları toplu bir iş olarak çalıştırır.

`type` `urlId` olarak ayarlandığında, yalnızca `urlId`'si tam olarak `from` olan sayfa taşınır, bu yüzden `post-1` `post-10` ile eşleşmez. Kontrol paneli aracı bu seçeneğe sahip değildir.

Her API taşıması, [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post) ile 90 gün içinde geri alınabilir.

---