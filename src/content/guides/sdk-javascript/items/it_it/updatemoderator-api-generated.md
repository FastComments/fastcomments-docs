## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateModeratorBody | UpdateModeratorBody | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-81';
const id: string = 'mod_7f3a2b';
const updateModeratorBody: UpdateModeratorBody = {
  email: 'j.reyes@acme-corp.com',
  displayName: 'Jordan Reyes',
  roles: ['moderator', 'content_reviewer'],
  active: true,
  notes: 'Promoted to senior moderator; monitor flagged content weekly'
};
const result: FlagCommentPublic200Response = await updateModerator(tenantId, id, updateModeratorBody);
[inline-code-end]

---