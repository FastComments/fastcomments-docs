## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| fromName | string | Da |  |

## Odgovor

Vrača: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer sendInvite'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_acme_42";
  const id: string = "cmt_8f3b21";
  const fromName: string = "Ava Thompson";
  const inviteResult: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName);
  console.log(inviteResult);
})();
[inline-code-end]

---