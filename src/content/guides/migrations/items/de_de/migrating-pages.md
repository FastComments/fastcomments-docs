---
Obwohl es den Namen Domain Migration Tool trägt, kann es auch verwendet werden, um Kommentare über einzelne Seiten hinweg zu migrieren.

Geben Sie einfach die alten und neuen [urlId](/guide-customizations-and-configuration.html#url-id) Werte in die Eingabefelder `from` und `to` ein.

### Seitenmigration über API

Um viele Seiten auf einmal zu migrieren, verwenden Sie [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Es akzeptiert bis zu 100 `from`- und `to`-Paare pro Anfrage und führt sie als Batch-Job aus.

Wenn `type` auf `urlId` gesetzt ist, wird nur die Seite verschoben, deren `urlId` exakt `from` entspricht, sodass `post-1` nicht mit `post-10` übereinstimmt. Das Dashboard-Tool bietet diese Option nicht.

Jede API-Migration kann innerhalb von 90 Tagen mit [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post) rückgängig gemacht werden.

---