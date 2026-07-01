## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## 応答

戻り値: [`UnBlockUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockUserFromCommentResponse.ts)

## 例

[inline-code-attrs-start title = 'unBlockUserFromComment 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUnblock() {
  const tenantId: string = "acme-corp-tenant";
  const commentId: string = "cmt_9f8b7a6d";

  const params: UnBlockFromCommentParams = {
    reason: "User resolved the issue",
    notifyUser: true
  };

  const userId: string = "usr_12345";

  const result: UnBlockUserFromCommentResponse = await unBlockUserFromComment(
    tenantId,
    commentId,
    params,
    userId
    // anonUserId は省略
  );

  console.log(result);
}
demoUnblock();
[inline-code-end]