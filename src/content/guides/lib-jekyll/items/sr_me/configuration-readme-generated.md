Config потиче из три места. Накнадни извори имају предност:

1. **Глобалне подразумеване вредности** у `_config.yml` под кључем `fastcomments:`.
2. **Контекст странице**, аутоматски изведен за виџете везане за страницу (погледајте у наставку).
3. **Атрибути тега** написани директно на самом тагу.

Дакле, `url_id` на тагу има предност над вредношћу изведеном са странице, која има предност над било којом глобалном подразумеваном вредношћу.

### Attribute syntax

Атрибути су парови `key=value` у `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Цитиране** вредности (`"..."` или `'...'`) су дословни низови.
- **Без навода** `true`/`false` постају булове, а бројеви постају бројеви.
- **Без навода** све остало се тумачи као Liquid променљива из контекста странице, нпр. `url_id=page.slug`. (Liquid не проширује `{% raw %}\{{ ... }}{% endraw %}` унутар атрибута тега, па користите голи облик `page.slug` уместо `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Кључеви у snake_case за атрибуте и конфигурацију аутоматски се мапирају на camelCase кључеве које FastComments очекује (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, итд.). Било која друга опција из [widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html) пролази на исти начин.

### Page-derived values

За виџете везане за страницу (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) ове вредности се аутоматски попуњавају из текуће странице осим ако их не подесите сами:

- `url_id` ← `page.url` (стабилан идентификатор независан од домена посетилаца)
- `url` ← `site.url` + `page.url` (само када је `url` подешен у `_config.yml`)
- `page_title` ← `page.title`

Виджети који покривају цео сајт (скори коментари/дискусије, најпопуларније странице, сажетак рецензија, фид активности корисника, bulk count) нису везани за страницу и не изводе ове вредности.

### EU data residency

Корисници у ЕУ додају `region: eu`, или глобално:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

или по тагу: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Виџети затим учитавају са ЕУ CDN-а.