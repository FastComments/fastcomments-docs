[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Décrit les informations d’identification effectuant la requête : le locataire auquel elles appartiennent et, pour les jetons d’accès OAuth, l’utilisateur qui a autorisé l’application. Les intégrations l’utilisent pour tester une connexion et l’étiqueter.

Avec une clé API, la réponse identifie uniquement le locataire. Avec un jeton d’accès OAuth, elle transporte également l’utilisateur autorisant ainsi que les étendues (scopes) qui ont été accordées.

[inline-code-attrs-start title = 'Exemple cURL Me'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de réponse Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** How the request was authenticated. **/
    authType: 'api-key' | 'oauth'
    /** The scopes the credential holds. API keys hold both. **/
    scopes: ('read' | 'write')[]
    /** Only present for OAuth tokens. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]