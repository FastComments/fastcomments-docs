## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 回應

返回：[`GetVotesForUserResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getVotesForUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "acme-corp";
  const urlId: string = "post-9f8b7c";
  const userId: string = "user-42";
  const anonUserId: string = "anon-123";

  const votesRequiredOnly: GetVotesForUserResponse1 = await getVotesForUser(tenantId, urlId);
  const votesWithUserId: GetVotesForUserResponse1 = await getVotesForUser(tenantId, urlId, userId);
  const votesWithAnonId: GetVotesForUserResponse1 = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
}
demo();
[inline-code-end]