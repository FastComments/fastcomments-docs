---
Aunque su nombre es la herramienta de Migración de Dominio, también puede usarse para migrar comentarios entre páginas individuales.

Simplemente ingrese los valores antiguos y nuevos de [urlId](/guide-customizations-and-configuration.html#url-id) en los campos de entrada `from` y `to`.

### Migrando páginas vía API

Para migrar muchas páginas a la vez, use [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Acepta hasta 100 `from` y `to` pares por solicitud y los ejecuta como un trabajo por lotes.

Con `type` establecido en `urlIdUrl`, solo se mueve la página cuyo `urlId` es exactamente `from`, por lo que `post-1` no coincide con `post-10`. La herramienta del panel no tiene esta opción.

Cada migración de API puede deshacerse durante 90 días con [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).

---