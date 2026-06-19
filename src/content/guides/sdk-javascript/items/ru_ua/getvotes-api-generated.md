## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Ответ

Возвращает: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример использования getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_8421';
const urlId: string | undefined = 'posts/2026/06/typescript-api-examples';
const votes: GetVotesResponse = await getVotes(tenantId, urlId!);
[inline-code-end]

---