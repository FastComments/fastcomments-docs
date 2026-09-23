## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'postUnFlagComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "comment_98765";
  const broadcastId: string = "broadcast_55555";
  const sso: string = "sso_token_abcde";

  const result1: APIEmptyResponse = await postUnFlagComment(tenantId, commentId);
  const result2: APIEmptyResponse = await postUnFlagComment(tenantId, commentId, broadcastId, sso);
}

runExample();
[inline-code-end]