## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Respons

Retourneert: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUser200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getVotesForUser Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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