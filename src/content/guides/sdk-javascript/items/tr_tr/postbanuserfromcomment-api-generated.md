## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| banEmail | boolean | Hayır |  |
| banEmailDomain | boolean | Hayır |  |
| banIP | boolean | Hayır |  |
| deleteAllUsersComments | boolean | Hayır |  |
| bannedUntil | string | Hayır |  |
| isShadowBan | boolean | Hayır |  |
| updateId | string | Hayır |  |
| banReason | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Örnek

[inline-code-attrs-start title = 'postBanUserFromComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (atlandı)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (atlandı)
    "Harassment",             // banReason
    undefined                 // sso (atlandı)
  );

  console.log(result);
}
[inline-code-end]

---