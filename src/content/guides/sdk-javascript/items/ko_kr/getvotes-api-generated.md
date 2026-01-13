## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotes200Response.ts)

## 예제

[inline-code-attrs-start title = 'getVotes 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3c_prod';
const urlId: string = '/news/2026/typescript-ecosystem-update';
const votes: GetVotes200Response = await getVotes(tenantId, urlId);
// 선택적 매개변수가 존재한다면, 예: includeHidden, 다음과 같이 사용할 수 있습니다:
// const votesWithHidden: GetVotes200Response = await getVotes(tenantId, urlId, { includeHidden: true });
[inline-code-end]