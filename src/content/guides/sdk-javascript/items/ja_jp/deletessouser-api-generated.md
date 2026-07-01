## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | boolean | No |  |
| commentDeleteMode | string | No |  |

## レスポンス

戻り値: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSSOUserAPIResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteSSOUser 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const userId: string = "sso-user-42";
const deleteComments: boolean = true;
const commentDeleteMode: string = "hard";

const detailedResult: DeleteSSOUserAPIResponse = await deleteSSOUser(
  tenantId,
  userId,
  deleteComments,
  commentDeleteMode
);

const simpleResult: DeleteSSOUserAPIResponse = await deleteSSOUser(
  tenantId,
  userId
);
[inline-code-end]