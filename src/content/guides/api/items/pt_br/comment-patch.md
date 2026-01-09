[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Este endpoint da API fornece a capacidade de atualizar um único comentário.

Notes:

- This API can update the comment widget "live" if desired (this increases base `creditsCost` from `1` to `2`).
  - This can make migrating comments between pages "live" (changing `urlId`).
  - Migrations cost an additional `2` credits as pages are precalculated and this is CPU intensive.
- Unlike the create API, this API will NOT automatically create user objects in our system if email is provided.
- Comments updated via this API can still be checked for spam if desired.
- Configuration such as max comment length, if configured via the Customization Rule admin page, will apply here.
- To allow users to update their comment text, you can just specify `comment` in the request body. We will generate the resulting `commentHTML`.
  - If you define both `comment` and `commentHTML` we will not automatically generate the HTML.
  - If the user adds mentions or hashtags in their new text, it will still be processed like the `POST` API.
- When updating `commenterEmail` on a comment, it is best to also specify `userId`. Otherwise, you must ensure the user with this email belongs to your tenant, or the request will fail.  


[inline-code-attrs-start title = 'Exemplo mínimo de cURL PATCH de Comentário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição PATCH de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** O usuário que está fazendo a atualização. Se desejado, pode ser usado para verificar que ele pode editar o comentário.  **/
    contextUserId?: string
	/** Devemos verificar se o novo comentário parece spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Se o comentário deve aparecer "ao vivo" para usuários que visualizam instâncias do widget de comentários com o mesmo urlId. NOTA: Dobra o custo de créditos de 1 para 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta PATCH de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Incluído em caso de falha. **/
    reason?: string
}
[inline-code-end]

---