Wszystkie opcje widżetu FastComments są ustawiane w sekcji `[params.fastcomments]` w pliku `hugo.toml` i mogą być nadpisywane per strona w front matter w sekcji `[fastcomments]`. Pierwszeństwo, od najniższego do najwyższego: parametry witryny, front matter strony, parametry shortcode.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# front matter strony
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Gdy nie podano ani `url`, ani `urlId`, `url` domyślnie przyjmuje permalink strony, dzięki czemu wątki komentarzy pozostają powiązane ze stabilnym adresem URL.

### Rezydencja danych w UE

Klienci z UE ustawiają `region = "eu"`, aby kierować widżet do `cdn-eu.fastcomments.com`:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Uwaga dotycząca wielkości liter w kluczach

Hugo zamienia wszystkie klucze w `hugo.toml` i front matter na małe litery, ale widżety FastComments wymagają kluczy w formacie camelCase (`tenantId`, `hasDarkBackground`). Ten komponent automatycznie przywraca poprawną wielkość liter dla każdej znanej opcji najwyższego poziomu, więc zapisuj opcje w ich normalnej formie camelCase. Klucze zagnieżdżone wewnątrz wartości obiektu (na przykład klucze mapy `translations` lub pola `pageReactConfig`) nie są przywracane. Skonfiguruj je zamiast tego przez interfejs dostosowywania w panelu FastComments.