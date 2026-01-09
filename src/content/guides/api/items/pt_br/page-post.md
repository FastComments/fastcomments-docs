[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Este endpoint da API fornece a capacidade de criar páginas.

Um caso de uso comum é controle de acesso.

Notas:

- Se você comentou em um thread de comentários, ou chamou a API para criar um `Comment`, você já criou um objeto `Page`! Você pode tentar recuperá-lo via
  a rota `Page` `/by-url-id`, passando o mesmo `urlId` passado para o widget de comentários.
- A `Page` structure contém alguns valores **calculados**.
  Atualmente, estes são `commentCount` e `rootCommentCount`.
  Eles são populados automaticamente e não podem ser definidos pela API. Tentar fazê-lo fará com que a API retorne um erro.

[inline-code-attrs-start title = 'Exemplo de cURL (POST) para Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição POST de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta POST de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** Incluído em caso de falha. **/
    reason?: string
    /** A página criada. **/
    page?: Page
}
[inline-code-end]

---