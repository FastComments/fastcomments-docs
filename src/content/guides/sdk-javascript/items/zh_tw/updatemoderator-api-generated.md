## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateModeratorBody | UpdateModeratorBody | 是 |  |

## 回應

回傳：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateModerator 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-corp-tenant-92';
  const id: string = '9f3b2c1a-4d6e-11ec-81d3-0242ac130003';
  const updateModeratorBody: UpdateModeratorBody = {
    email: 'moderator.lead@acmecorp.com',
    displayName: 'Alex Rivera',
    roles: ['moderator', 'team_lead'],
    active: true,
    notify: true // 可選參數，用以通知版主有關變更
  };
  const result: APIEmptyResponse = await updateModerator(tenantId, id, updateModeratorBody);
  console.log(result);
})();
[inline-code-end]

---