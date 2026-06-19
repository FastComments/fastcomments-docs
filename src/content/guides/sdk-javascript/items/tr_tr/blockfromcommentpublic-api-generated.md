## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## Örnek

[inline-code-attrs-start title = 'blockFromCommentPublic Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_52b9f3a1";
const commentId: string = "cmt_4f9d2a7b";
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: "spam",
  moderatorId: "mod_783",
  durationMinutes: 1440,
  notifyUser: true
};
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example";
const result: BlockSuccess = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---