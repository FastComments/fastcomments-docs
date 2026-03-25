## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userId | string | 아니요 |  |
| anonUserId | string | 아니요 |  |

## 응답

반환: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUser200Response.ts)

## 예제

[inline-code-attrs-start title = 'getVotesForUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async (): Promise<void> => {
  const tenantId: string = "local-news-ny";
  const urlId: string = "articles/2026-03-25/ev-infrastructure-update";
  const userId: string = "user_78b6f3d9";
  const anonUserId: string = "anon_9c3f7a1b";
  const result: GetVotesForUser200Response = await getVotesForUser(tenantId, urlId, userId, anonUserId);
  console.log(result);
})();
[inline-code-end]

---