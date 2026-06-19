## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateModeratorBody | UpdateModeratorBody | Yes |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-corp-tenant-92';
  const id: string = '9f3b2c1a-4d6e-11ec-81d3-0242ac130003';
  const updateModeratorBody: UpdateModeratorBody = {
    email: 'moderator.lead@acmecorp.com',
    displayName: 'Alex Rivera',
    roles: ['moderator', 'team_lead'],
    active: true,
    notify: true // דגל אופציונלי להודיע למנחה על שינויים
  };
  const result: APIEmptyResponse = await updateModerator(tenantId, id, updateModeratorBody);
  console.log(result);
})();
[inline-code-end]

---