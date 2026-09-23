## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|------------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| banEmail | boolean | Nee |  |
| banEmailDomain | boolean | Nee |  |
| banIP | boolean | Nee |  |
| deleteAllUsersComments | boolean | Nee |  |
| bannedUntil | string | Nee |  |
| isShadowBan | boolean | Nee |  |
| updateId | string | Nee |  |
| banReason | string | Nee |  |
| sso | string | Nee |  |

## Response

Retourneert: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Example

[inline-code-attrs-start title = 'postBanUserFromComment voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (weggelaten)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (weggelaten)
    "Harassment",             // banReason
    undefined                 // sso (weggelaten)
  );

  console.log(result);
}
[inline-code-end]