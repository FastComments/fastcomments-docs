## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Ja |  |
| updateComments | boolean | Nein |  |

## Antwort

Gibt zurück: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutSSOUserAPIResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'putSSOUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-enterprises-42';
const id: string = 'usr-73a1b2';
const updateAPISSOUserData: UpdateAPISSOUserData = {
  email: 'marcus.ingram@acme.com',
  givenName: 'Marcus',
  familyName: 'Ingram',
  roles: ['editor', 'project_owner'],
  enabled: true
};
const result: PutSSOUserAPIResponse = await putSSOUser(tenantId, id, updateAPISSOUserData, true);
[inline-code-end]

---