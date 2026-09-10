[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

Opisuje poświadczenie wykonujące żądanie: najemcę, do którego należy, oraz w przypadku tokenów OAuth,
użytkownika, który autoryzował aplikację. Integracje używają go do testowania połączenia i nadawania mu etykiety.

Przy użyciu klucza API odpowiedź identyfikuje tylko najemcę. Przy tokenie OAuth przekazywany jest również
użytkownik autoryzujący oraz zakresy, które zostały przyznane.

[inline-code-attrs-start title = 'Przykład cURL Me'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi Me'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    tenantId: string
    tenantName: string
    /** Jak żądanie zostało uwierzytelnione. **/
    authType: 'api-key' | 'oauth'
    /** Zakresy, które posiada poświadczenie. Klucze API posiadają oba. **/
    scopes: ('read' | 'write')[]
    /** Obecne tylko dla tokenów OAuth. **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]