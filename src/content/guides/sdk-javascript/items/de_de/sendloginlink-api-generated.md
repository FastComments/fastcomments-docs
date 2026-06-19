## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| redirectURL | string | Nein |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'sendLoginLink Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async function run(): Promise<void> {
  const tenantId: string = 'fc_tenant_9f3b2c';
  const id: string = 'user_42b7f';
  const redirectURL: string = 'https://dashboard.acme-corp.com/welcome';
  const responseWithoutRedirect: APIEmptyResponse = await sendLoginLink(tenantId, id);
  const responseWithRedirect: APIEmptyResponse = await sendLoginLink(tenantId, id, redirectURL);
  console.log(responseWithoutRedirect, responseWithRedirect);
})();
[inline-code-end]

---