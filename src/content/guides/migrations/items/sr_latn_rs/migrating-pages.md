Iako se zove Domain Migration alat, može se koristiti i za migraciju komentara između pojedinačnih stranica.

Jednostavno unesite stare i nove [urlId](/guide-customizations-and-configuration.html#url-id) vrednosti u `from` i `to` polja.

### Migracija stranica putem API-ja

Za migraciju više stranica odjednom, koristite [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Prihvata do 100 `from` i `to` parova po zahtevu i izvršava ih kao batch posao.

Kada je `type` postavljeno na `urlId`, premešta se samo stranica čiji je `urlId` tačno `from`, tako da `post-1` ne odgovara `post-10`. Alat u kontrolnom panelu nema ovu opciju.

Svaku API migraciju je moguće poništiti u roku od 90 dana pomoću [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).