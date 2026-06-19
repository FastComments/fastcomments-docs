## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| fromName | string | Sim |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de sendInvite'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'bright-media-12';
const id: string = 'user-8f4d2';
const fromName: string = 'Sofia Park';
const optionalNote: string | undefined = undefined;
const result: APIEmptyResponse = await sendInvite(tenantId, id, fromName);
[inline-code-end]

---