Čeprav se orodje imenuje Domain Migration, ga je mogoče uporabiti tudi za selitev komentarjev med posameznimi stranmi.

Preprosto vnesite stare in nove vrednosti [urlId](/guide-customizations-and-configuration.html#url-id) v vnosna polja `from` in `to`.

### Selitev strani prek API-ja

Za hkratno selitev več strani uporabite [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Sprejme do 100 parov `from` in `to` na zahtevo ter jih izvede kot paketno opravilo.

Ko je `type` nastavljen na `urlId`, se premakne samo stran, katere `urlId` je točno `from`, zato `post-1` ne ustreza `post-10`. Orodje na nadzorni plošči te možnosti nima.

Vsako selitev prek API-ja je mogoče razveljaviti 90 dni s pomočjo [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).