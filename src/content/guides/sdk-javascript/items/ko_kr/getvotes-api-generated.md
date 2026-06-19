## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## 예제

[inline-code-attrs-start title = 'getVotes 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_8421';
const urlId: string | undefined = 'posts/2026/06/typescript-api-examples';
const votes: GetVotesResponse = await getVotes(tenantId, urlId!);
[inline-code-end]

---