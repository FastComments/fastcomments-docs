## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| banUserUndoParams | BanUserUndoParams | Yes |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 响应

返回: [`PostBanUserUndoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBanUserUndoResponse.ts)

## 示例

[inline-code-attrs-start title = 'postBanUserUndo 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const undoParams: BanUserUndoParams = {
  userId: "user-8421",
  commentId: "comment-6702",
  reason: "Accidental ban"
};

const tenantId: string = "tenant-7f9e";
const ssoToken: string = "sso-4b2c1d";

async function runUndo() {
  const resultAll: PostBanUserUndoResponse = await postBanUserUndo(undoParams, tenantId, ssoToken);
  console.log(resultAll);

  const resultMinimal: PostBanUserUndoResponse = await postBanUserUndo(undoParams);
  console.log(resultMinimal);
}

runUndo();
[inline-code-end]