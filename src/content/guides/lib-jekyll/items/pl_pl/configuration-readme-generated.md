---
Konfiguracja pochodzi z trzech miejsc. Późniejsze źródła mają pierwszeństwo:

1. **Domyślne ustawienia globalne** w `_config.yml` pod kluczem `fastcomments:`.
2. **Kontekst strony**, wyprowadzany automatycznie dla widgetów związanych ze stroną (patrz niżej).
3. **Atrybuty tagu** zapisane bezpośrednio w samym tagu.

Tak więc `url_id` na tagu nadpisuje wartość wyprowadzoną ze strony, która z kolei nadpisuje dowolne ustawienie globalne.

### Składnia atrybutów

Atrybuty to pary `key=value` w `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Cytowane** wartości (`"..."` lub `'...'`) są dosłownymi ciągami znaków.
- **Niecytowane** `true`/`false` stają się wartościami logicznymi (boolean), a liczby stają się liczbami.
- **Niecytowane** wszystko inne jest rozwiązywane jako zmienna Liquid z kontekstu strony, np. `url_id=page.slug`. (Liquid nie rozwija `{% raw %}\{{ ... }}{% endraw %}` wewnątrz atrybutów tagu, więc użyj formy `page.slug` zamiast `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Klucze atrybutów i konfiguracji w snake_case są automatycznie mapowane na klucze w camelCase, których oczekuje FastComments (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`, `has_dark_background` → `hasDarkBackground` i tak dalej). Każda inna opcja z [konfiguracji widgetu](https://docs.fastcomments.com/guide-customizations-and-configuration.html) przechodzi w ten sam sposób.

### Wartości pochodzące ze strony

Dla widgetów związanych ze stroną (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`) są one wypełniane automatycznie z bieżącej strony, chyba że ustawisz je samodzielnie:

- `url_id` ← `page.url` (stabilny identyfikator niezależny od odwiedzającej domeny)
- `url` ← `site.url` + `page.url` (tylko gdy `url` jest ustawiony w `_config.yml`)
- `page_title` ← `page.title`

Widgety ogólnostronne (najnowsze komentarze/dyskusje, najważniejsze strony, podsumowanie recenzji, kanał aktywności użytkowników, zbiorcze liczniki) nie są powiązane ze stroną i nie pozyskują tych wartości.

### Lokalizacja danych w UE

Klienci z UE dodają `region: eu`, albo globalnie:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

lub dla poszczególnego tagu: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Widgety będą wtedy ładowane z europejskiego CDN.
---