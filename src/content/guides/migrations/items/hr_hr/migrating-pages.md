---
Iako se zove alat za migraciju domena, također se može koristiti za migriranje komentara kroz pojedinačne stranice.

Jednostavno unesite stare i nove [urlId](/guide-customizations-and-configuration.html#url-id) vrijednosti u polja `from` i `to`.

### Migriranje stranica putem API-ja

Za migriranje mnogih stranica odjednom, koristite [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Uzima do 100 parova `from` i `to` po zahtjevu i izvršava ih kao batch posao.

Kada je `type` postavljen na `urlId`, samo se stranica čiji je `urlId` točno `from` premješta, pa `post-1` ne odgovara `post-10`. Alat na nadzornoj ploči nema ovu opciju.

Svaku API migraciju moguće je poništiti u roku od 90 dana pomoću [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).

---