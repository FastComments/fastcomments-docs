[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Cette API utilise la pagination, fournie par les paramètres `skip`, `limit`, `before` et `after`. Les AuditLogs sont renvoyés par pages de `5000` par défaut, jusqu'à une `limit` maximale de `10000`, triés par `when` et `id`. Les pages sont volumineuses car ce point de terminaison est généralement utilisé pour extraire l'historique plutôt que pour le parcourir de façon interactive.

Chaque tranche de `100` journaux renvoyés coûte `1` crédit.

Par défaut, vous recevrez une liste avec **les éléments les plus récents en premier**. Ainsi, vous pouvez interroger en commençant par `skip=0`, en paginant jusqu'à ce que vous trouviez le dernier enregistrement que vous avez consommé.

Alternativement, vous pouvez trier du plus ancien au plus récent, et paginer jusqu'à ce qu'il n'y ait plus d'enregistrements.

Le tri peut être effectué en définissant `order` sur `ASC` ou `DESC`. La valeur par défaut est `DESC`.

Il est possible de filtrer par date via `before` et `after` en tant qu'horodatages en millisecondes. `before` et `after` ne sont PAS inclusifs, et chacun peut être utilisé seul.

## Trouver ce qui est arrivé à une personne

Chaque événement enregistre qui l'a effectué (`username`, `userId`, `ip`) et, séparément, sur quoi il a été effectué. `targetLabel` est une étiquette lisible par l'homme pour cet objet, par exemple `jsmith (jsmith@example.com)`, et `targetId` est son identifiant. Utilisez `target` pour une correspondance insensible à la casse sur l'étiquette lorsque vous connaissez le nom ou l'email d'une personne mais pas son identifiant.

Les suppressions capturent l'étiquette au moment de l'événement, de sorte qu'un utilisateur ou modérateur supprimé puisse encore être identifié après la disparition de l'enregistrement sous-jacent.

## Locataires gérés

Si votre locataire gère d'autres locataires, définissez `includeManagedTenants=true` pour renvoyer les événements de votre locataire et de chaque locataire qu'il gère dans une seule réponse. Le `tenantId` de chaque journal renvoyé indique de quel locataire il provient.

[inline-code-attrs-start title = 'Exemple cURL d\'AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Max 10000. Defaults to 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Only events performed by this username. **/
    username?: string
    /** Only events from this IP address. **/
    ip?: string
    /** Only events of this type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Only events for this resource, e.g. User or Moderator. **/
    resourceName?: string
    /** Only events whose affected object has this id. **/
    targetId?: string
    /** Case-insensitive substring match on the affected object's label. **/
    target?: string
    /** Also return events from tenants this tenant manages. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]