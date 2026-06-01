Config üç yerden gelir. Daha sonraki kaynaklar kazanır:

1. **Genel varsayılanlar** `_config.yml` içinde `fastcomments:` anahtarının altında.
2. **Sayfa bağlamı**, sayfa kapsamlı widget'lar için otomatik olarak türetilir (aşağıya bakın).
3. **Etiket üzerinde yazılan öznitelikler**.

Dolayısıyla etiketteki `url_id`, sayfadan türetilen değerin üzerine yazar; bu da herhangi bir genel varsayılanın üzerine yazar.

### Öznitelik sözdizimi

Öznitelikler `snake_case` içinde `key=value` çiftleridir:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Alıntılanmış** değerler (`"..."` veya `'...'`) literal string’lerdir.
- **Alıntılanmamış** `true`/`false` booleana dönüşür ve sayılar sayı olur.
- **Alıntılanmamış** diğer herhangi bir şey sayfa bağlamından bir Liquid değişkeni olarak çözülür, örn.
  `url_id=page.slug`. (Liquid, bir etiketin öznitelikleri içinde `{% raw %}\{{ ... }}{% endraw %}` genişletmez, bu yüzden `"{% raw %}\{{ page.slug }}{% endraw %}"` yerine çıplak `page.slug` biçimini kullanın.)

Snake_case öznitelik ve konfigürasyon anahtarları FastComments'ın beklediği camelCase anahtarlara otomatik olarak eşlenir (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground` vb.). [widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html) içindeki herhangi bir başka seçenek de aynı şekilde doğrudan geçer.

### Sayfadan türetilen değerler

Sayfa kapsamlı widget'lar (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) için bunlar kendi kendine mevcut sayfadan otomatik olarak doldurulur, siz kendiniz ayarlamadığınız sürece:

- `url_id` ← `page.url` (ziyaret edilen alan adından bağımsız sabit bir tanımlayıcı)
- `url` ← `site.url` + `page.url` (`url` yalnızca `_config.yml` içinde ayarlanmışsa)
- `page_title` ← `page.title`

Site genelindeki widget'lar (son yorumlar/tartışmalar, en iyi sayfalar, incelemeler özeti, kullanıcı etkinlik akışı, toplu sayım) bir sayfaya bağlı değildir ve bunları türetmez.

### AB veri yerleşimi

AB müşterileri `region: eu` ekler, ya genel olarak:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

veya etiket bazında: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Widget'lar daha sonra AB CDN'inden yüklenir.