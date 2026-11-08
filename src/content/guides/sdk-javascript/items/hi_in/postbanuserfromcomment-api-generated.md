## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| banEmail | boolean | नहीं |  |
| banEmailDomain | boolean | नहीं |  |
| banIP | boolean | नहीं |  |
| deleteAllUsersComments | boolean | नहीं |  |
| bannedUntil | string | नहीं |  |
| isShadowBan | boolean | नहीं |  |
| updateId | string | नहीं |  |
| banReason | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## उदाहरण

[inline-code-attrs-start title = 'postBanUserFromComment उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (छोड़ा गया)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (छोड़ा गया)
    "Harassment",             // banReason
    undefined                 // sso (छोड़ा गया)
  );

  console.log(result);
}
[inline-code-end]