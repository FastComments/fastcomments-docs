---
Wszystkie opcje widżetu FastComments ustawiane są w sekcji `[params.fastcomments]` w `hugo.toml`, i mogą być nadpisane dla każdej strony we front matter pod `[fastcomments]`. Kolejność priorytetu, od najniższego do najwyższego: parametry witryny, front matter strony, parametry shortcode.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

When neither `url` nor `urlId` is provided, `url` defaults to the page's permalink so comment threads stay tied to a stable URL.

### Rezydencja danych w UE

Klienci z UE ustawiają `region = "eu"` aby kierować widżet do `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Uwaga dotycząca wielkości liter kluczy

Hugo zamienia na małe litery każdy klucz w `hugo.toml` i we front matter, ale widżety FastComments wymagają kluczy w formacie camelCase (`tenantId`, `hasDarkBackground`). Ten komponent automatycznie przywraca poprawne odmiany wielkości liter dla każdej znanej opcji najwyższego poziomu, więc wpisuj opcje w ich normalnej formie camelCase. Klucze zagnieżdżone wewnątrz wartości obiektu (na przykład klucze mapy `translations` lub pola `pageReactConfig`) nie są przywracane. Skonfiguruj je zamiast tego poprzez interfejs dostosowywania w panelu FastComments.
---