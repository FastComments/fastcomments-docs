---
Chociaż nazywa się narzędziem do migracji domen, może być również używane do przenoszenia komentarzy pomiędzy poszczególnymi stronami.

Po prostu wprowadź stare i nowe wartości [urlId](/guide-customizations-and-configuration.html#url-id) w pola wejściowe `from` i `to`.

### Migracja stron za pomocą API

Aby migrować wiele stron jednocześnie, użyj [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Przyjmuje do 100 par `from` i `to` na żądanie i uruchamia je jako zadanie wsadowe.

Gdy `type` jest ustawiony na `urlId`, przenoszona jest tylko strona, której `urlId` jest dokładnie `from`, więc `post-1` nie pasuje do `post-10`. Narzędzie w panelu nie posiada tej opcji.

Każdą migrację API można cofnąć w ciągu 90 dni przy użyciu [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).

---