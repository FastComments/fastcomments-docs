## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## תגובה

מחזיר: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-67890";

(async () => {
  const votesResponse: GetVotesResponse = await getVotes(tenantId, urlId);
  // דוגמה לגישה למאפיין אופציונלי מהתגובה
  const firstVote: PublicVote | undefined = votesResponse.votes?.[0];
})();
[inline-code-end]