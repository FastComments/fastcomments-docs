[api-resource-header-start name = 'NotificationCount'; route = 'DELETE /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

Cette route supprime un seul `NotificationCount` par id utilisateur. Avec SSO, l'id utilisateur est au format `<tenant id>:<user id>`.

Cela effacera le compteur de notifications non lues de l'utilisateur (la cloche rouge dans le widget de commentaires s'estompera et le compteur disparaîtra).

[inline-code-attrs-start title = 'Exemple cURL de DELETE NotificationCount'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success"}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Requête de Suppression de NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de Réponse de Suppression de NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
