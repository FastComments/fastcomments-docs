## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | boolean | No |  |
| commentDeleteMode | string | No |  |

## 回應

返回：[`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSSOUserAPIResponse.ts)

## 範例

[inline-code-attrs-start title = 'deleteSSOUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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