While its name is the Domain Migration tool, it can also be used to migrate comments across individual pages.

Preprosto vnesite stare in nove vrednosti [urlId](/guide-customizations-and-configuration.html#url-id) v polja za vnos `from` in `to`.

### Migriranje strani prek API-ja

Za selitev več strani hkrati uporabite [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Sprejme do 100 parov `from` in `to` na zahtevo ter jih izvede kot paketno opravilo.

Ko je `type` nastavljen na `urlId`, se premakne le stran, katere `urlId` je točno `from`, zato `post-1` ne ustreza `post-10`. Orodje na nadzorni plošči te možnosti nima.

Vsako selitev prek API-ja je mogoče razveljaviti v roku 90 dni z [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).