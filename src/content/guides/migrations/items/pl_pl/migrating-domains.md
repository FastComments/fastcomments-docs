FastComments zapewnia zautomatyzowany sposób na migrację Twoich komentarzy pomiędzy domenami.

Migracja domen wymaga po prostu domeny `from` i domeny `to`.

To **przenosi** komentarze, nie kopiuje ich. Jeśli chcesz skopiować komentarze, skontaktuj się z nami.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Narzędzie do migracji domen z polami from i to oraz liczbą przeniesionych komentarzy'; title='Migracja domen' app-screenshot-end]

Jest to również przydatne, na przykład jeśli część Twojej migracji do FastComments obejmuje migrację z innego dostawcy, więc dane importu komentarzy
mogą zawierać informacje, które trzeba migrować. W takim przypadku możesz najpierw uruchomić import, a następnie migrację domen.

### Monitorowanie postępu

Narzędzie do migracji domen używa tego samego systemu przetwarzania zadań FastComments co inne narzędzia zarządzania danymi.

Może wystąpić opóźnienie przed rozpoczęciem migracji. Jest to normalne, ponieważ system okresowo sprawdza nowe zadania do przetworzenia.

Podczas działania zadania wyświetlana będzie liczba komentarzy znalezionych do migracji oraz liczba dotychczas przeniesionych.

### Migracja domen za pomocą API

Domeny można również migrować przy użyciu [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), ustawiając `type` na `domain`. Postęp jest dostępny pod adresem [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), a migrację można cofnąć przez 90 dni.

---