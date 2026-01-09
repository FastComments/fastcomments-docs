[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Este endpoint da API fornece a capacidade de criar comentários.

Casos de uso comuns incluem UIs personalizadas, integrações ou importações.

Notas:

- Esta API pode atualizar o widget de comentários "live" se desejado (isso aumenta o `creditsCost` de `1` para `2`).
- Esta API criará automaticamente objetos de usuário em nosso sistema se o email for fornecido.
- Tentar salvar dois comentários com emails diferentes, mas o mesmo nome de usuário, resultará em um erro para o segundo comentário.
- Se você estiver especificando `parentId`, e um comentário filho tiver `notificationSentForParent` como false, **enviaremos notificações para o comentário pai**. Isso é feito a cada hora (agrupamos as notificações para diminuir o número de emails enviados).
- Se você quiser enviar emails de boas-vindas ao criar usuários, ou emails de verificação de comentário, defina `sendEmails` como `true` nos parâmetros de consulta.
- Comentários criados via esta API aparecerão nas páginas de Analytics e Moderation do aplicativo de administração.
- Palavrões ainda são mascarados nos nomes dos comentaristas e no texto do comentário se a configuração estiver ativada.
- Comentários criados via esta API ainda podem ser verificados quanto a spam, se desejado.
- Configurações como comprimento máximo do comentário, se configuradas via a página de administração Customization Rule, serão aplicadas aqui.

Os dados mínimos necessários para enviar que serão exibidos no widget de comentários são os seguintes:

[inline-code-attrs-start title = 'Exemplo mínimo de POST cURL de Comentário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Uma requisição mais realista pode ser:

[inline-code-attrs-start title = 'Exemplo de POST cURL para Comentário'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição POST de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Se o comentário deve aparecer "live" para usuários visualizando instâncias do widget de comentários com o mesmo urlId. NOTA: Dobra o custo de créditos de 1 para 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta POST de Comentário'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Incluído em caso de falha. **/
    reason?: string
    /** O comentário criado. **/
    comment?: Comment
    /** O usuário associado, que pode ou não já ter existido. **/
    user?: User
}
[inline-code-end]

---