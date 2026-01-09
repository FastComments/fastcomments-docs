[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

Esta rota fornece a capacidade de atualizar uma única `Page`. Os comentários correspondentes serão atualizados.

[inline-code-attrs-start title = 'Exemplo cURL de atualização de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de requisição de atualização de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de resposta de atualização de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** Incluído em caso de falha. **/
    reason?: string
    user?: Page; // Retornamos a página atualizada completa em caso de sucesso.
}
[inline-code-end]

#### Nota

Alguns parâmetros no objeto Page são atualizados automaticamente. Estes são os atributos counts e title. Os counts não podem ser atualizados
via a API já que são valores calculados. O `title` da página pode ser definido via a API, mas seria sobrescrito se o widget de comentários for usado em
uma página com o mesmo `urlId` e um título de página diferente.