[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Esta ruta mueve comentarios y páginas de un `urlId` o dominio a otro, de la misma forma que la [herramienta de migración de dominios](/guide-migrations.html#migrating-domains) en el panel. Se pueden enviar hasta 100 migraciones por solicitud. Se ejecutan en el orden dado, por lo que `a` a `b` seguido de `b` a `c` funciona.

Cada migración tiene un `type`:

- `urlId` mueve la página cuyo `urlId` es exactamente `from`. `post-1` no coincide con `post-10`.
- `domain` busca/reemplaza `from` dentro de cada `urlId` y `url`, y actualiza el `domain` de cada comentario. Ambos valores deben ser un dominio, como `old.example.com`.

Los comentarios, la página, reacciones de la página, votos, notificaciones y suscripciones se trasladan todos. Si ya existe una página en `to`, las dos páginas se fusionan.

La migración se ejecuta como un trabajo por lotes. La respuesta contiene un `batchJobId`. Realice polling a [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) para seguir su progreso. Una vez que el trabajo termina, tiene un resultado por migración, y un `undoId` que puede pasar a [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) dentro de los 90 días.

La solicitud se valida antes de que se encole algo. Si alguna migración es inválida, toda la solicitud falla con un `422` y no se ejecuta nada.

Encolar una migración no cuesta nada. Cuando el trabajo termina, cuesta `1` crédito por cada `1000` comentarios migrados, con un mínimo de `1`.

[inline-code-attrs-start title = 'Ejemplo cURL de Migraciones de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Solicitud de Migraciones de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Run in the order given. At most 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** At most 1000 characters. Leading and trailing whitespace is removed. **/
    from: string
    /** Must be different from "from". At most 1000 characters. **/
    to: string
    /** "urlId" only matches the exact urlId. "domain" find/replaces the value inside every urlId and url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Respuesta de Migraciones Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Included on failure. For invalid-migration, this names the index of the first invalid migration. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]

#### Notas

- Los trabajos para un inquilino se ejecutan uno a la vez, del más antiguo al más reciente. Una segunda migración espera a que la primera termine.
- `PATCH /api/v1/pages/:id` con un nuevo `urlId` solo cambia la página, no sus comentarios. Use esta ruta para mover los comentarios de una página.

---