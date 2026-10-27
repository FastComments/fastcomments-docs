---
Hoewel het de Domain Migration-tool heet, kan het ook worden gebruikt om reacties over individuele pagina's te migreren.

Voer eenvoudig de oude en nieuwe [urlId](/guide-customizations-and-configuration.html#url-id) waarden in de invoervelden `from` en `to` in.

### Pagina's migreren via API

Om veel pagina's tegelijk te migreren, gebruik je [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Het accepteert tot 100 `from`- en `to`-paren per verzoek en voert ze uit als een batchtaak.

Met `type` ingesteld op `urlId` wordt alleen de pagina waarvan de `urlId` exact `from` is verplaatst, dus `post-1` komt niet overeen met `post-10`. De dashboardtool heeft deze optie niet.

Elke API-migratie kan gedurende 90 dagen ongedaan worden gemaakt met [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).

---