La configurazione proviene da tre posti. Le fonti successive hanno la precedenza:

1. **Predefiniti globali** in `_config.yml` sotto la chiave `fastcomments:`.
2. **Contesto della pagina**, derivato automaticamente per i widget a livello di pagina (vedi sotto).
3. **Attributi del tag** scritti sul tag stesso.

Quindi un `url_id` sul tag sovrascrive il valore derivato dalla pagina, che sovrascrive qualsiasi predefinito globale.

### Sintassi degli attributi

Gli attributi sono coppie `key=value` in `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **I valori tra virgolette** (`"..."` o `'...'`) sono stringhe letterali.
- **Non tra virgolette** `true`/`false` diventano booleani, e i numeri diventano numeri.
- **Non tra virgolette** qualsiasi altra cosa viene risolta come una variabile Liquid dal contesto della pagina, e.g.
  `url_id=page.slug`. (Liquid non espande `{% raw %}\{{ ... }}{% endraw %}` all'interno degli attributi di un tag,
  quindi usa la forma semplice `page.slug` invece di `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Le chiavi degli attributi e della configurazione in snake_case vengono mappate automaticamente sulle chiavi camelCase che FastComments si aspetta (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground`, e così via). Qualsiasi altra opzione dalla
[widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
passa direttamente nello stesso modo.

### Valori derivati dalla pagina

Per i widget con ambito pagina (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) questi vengono compilati automaticamente dalla pagina corrente a meno che non li imposti tu stesso:

- `url_id` ← `page.url` (un identificatore stabile indipendente dal dominio visitato)
- `url` ← `site.url` + `page.url` (solo quando `url` è impostato in `_config.yml`)
- `page_title` ← `page.title`

I widget a livello di sito (recent comments/discussions, top pages, reviews summary, user activity feed,
bulk count) non sono legati a una pagina e non derivano questi valori.

### Residenza dei dati nell'UE

I clienti dell'UE aggiungono `region: eu`, o globalmente:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

o per tag: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. I widget verranno quindi caricati dalla CDN UE.