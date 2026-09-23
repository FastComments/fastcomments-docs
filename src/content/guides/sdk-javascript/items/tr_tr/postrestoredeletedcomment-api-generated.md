## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| broadcastId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'postRestoreDeletedComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";

const resultRequired: APIEmptyResponse = await postRestoreDeletedComment(tenantId, commentId);

const broadcastId: string = "brd_001";
const sso: string = "sso_token_abc123";

const resultAll: APIEmptyResponse = await postRestoreDeletedComment(
  tenantId,
  commentId,
  broadcastId,
  sso
);
[inline-code-end]