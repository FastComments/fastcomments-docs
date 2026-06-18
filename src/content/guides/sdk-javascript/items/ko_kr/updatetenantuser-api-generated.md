## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantUserBody | UpdateTenantUserBody | 예 |  |
| updateComments | string | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'updateTenantUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const userId: string = 'u_72b9f4';
const updateTenantUserBody: UpdateTenantUserBody = {
  email: 'jane.doe@acme.com',
  displayName: 'Jane Doe',
  roles: ['moderator'],
  suspended: false
};
const updateComments: string = 'Promoted to moderator after review of activity and community feedback';
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, userId, updateTenantUserBody, updateComments);
[inline-code-end]

---