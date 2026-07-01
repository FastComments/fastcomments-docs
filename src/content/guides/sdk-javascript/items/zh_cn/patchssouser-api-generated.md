## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | boolean | No |  |

## 响应

返回：[`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchSSOUserAPIResponse.ts)

## 示例

[inline-code-attrs-start title = 'patchSSOUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1f2a3b4-5678-90ab-cdef-1234567890ab";
const userId: string = "user-987654321";
const updateData: UpdateAPISSOUserData = {
  email: "jane.smith@enterprise.com",
  displayName: "Jane Smith",
  role: "editor"
};

const responseWithComments: PatchSSOUserAPIResponse = await patchSSOUser(
  tenantId,
  userId,
  updateData,
  true
);

const responseWithoutComments: PatchSSOUserAPIResponse = await patchSSOUser(
  tenantId,
  userId,
  updateData
);
[inline-code-end]