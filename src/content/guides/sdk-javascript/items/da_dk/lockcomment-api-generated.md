## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'lockComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoLockComment() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_9876";
  const broadcastId: string = "brd_5555";
  const ssoToken: string = "sso_user_abc";

  const resultWithSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId, ssoToken);
  const resultWithoutSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId);
}

demoLockComment();
[inline-code-end]

---