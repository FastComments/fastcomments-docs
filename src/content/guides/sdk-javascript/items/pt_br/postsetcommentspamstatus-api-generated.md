## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| commentId | string | Sim |  |
| spam | boolean | Não |  |
| permNotSpam | boolean | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de postSetCommentSpamStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_9f8b3a2e';
const spam: boolean = false;
const permNotSpam: boolean = true;
const sso: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signedToken';
const result: APIEmptyResponse = await postSetCommentSpamStatus(commentId, spam, permNotSpam, sso);
[inline-code-end]

---