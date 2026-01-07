[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Cette API utilise la pagination, fournie par les paramètres `skip`, `before` et `after`. Les AuditLogs sont retournés par pages de `1000`, triés par `when` et `id`.

La récupération de chaque `1000` logs a un coût en crédits de `10`.

Par défaut, vous recevrez une liste avec **les éléments les plus récents en premier**. De cette façon, vous pouvez interroger en commençant par `skip=0`, en paginant jusqu'à trouver le dernier enregistrement que vous avez consommé.

Alternativement, vous pouvez trier du plus ancien au plus récent, et paginer jusqu'à ce qu'il n'y ait plus d'enregistrements.

Le tri peut être effectué en définissant `order` sur `ASC` ou `DESC`. La valeur par défaut est `ASC`.

L'interrogation par date est possible via `before` et `after` en tant que timestamps avec millisecondes. `before` et `after` ne sont PAS inclusifs.

[inline-code-attrs-start title = 'Exemple cURL AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]
