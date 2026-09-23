## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| commentId | string | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse.ts)

## Primer

[inline-code-attrs-start title = 'getUserInternalProfile Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_9876";
  const ssoToken: string = "sso_abcde12345";

  const profileOnly: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId);
  const profileWithComment: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId, commentId);
  const fullProfile: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId, commentId, ssoToken);
}

demo();
[inline-code-end]