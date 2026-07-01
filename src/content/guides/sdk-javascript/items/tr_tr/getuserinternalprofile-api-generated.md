## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Hayır |  |
| tenantId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetUserInternalProfileResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse1.ts)

## Örnek

[inline-code-attrs-start title = 'getUserInternalProfile Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const fullProfile: GetUserInternalProfileResponse1 = await getUserInternalProfile({
    commentId: "cmt_12345",
    tenantId: "tenant_67890",
    sso: "sso_token_abcdef"
  });

  const partialProfile: GetUserInternalProfileResponse1 = await getUserInternalProfile({
    commentId: "cmt_98765"
  });
})();
[inline-code-end]