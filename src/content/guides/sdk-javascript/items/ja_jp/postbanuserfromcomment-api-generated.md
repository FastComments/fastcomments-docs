## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | はい |  |
| banEmail | boolean | いいえ |  |
| banEmailDomain | boolean | いいえ |  |
| banIP | boolean | いいえ |  |
| deleteAllUsersComments | boolean | いいえ |  |
| bannedUntil | string | いいえ |  |
| isShadowBan | boolean | いいえ |  |
| updateId | string | いいえ |  |
| banReason | string | いいえ |  |
| tenantId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`PostBanUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBanUserFromCommentResponse.ts)

## 例

[inline-code-attrs-start title = 'postBanUserFromComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runBan() {
  const commentId: string = "cmt_5f8a2b3c";
  const banEmail: boolean = true;
  const banIP: boolean = false;
  const deleteAllUsersComments: boolean = true;
  const bannedUntil: string = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString();
  const isShadowBan: boolean = false;
  const banReason: string = "Repeated spam posting";
  const tenantId: string = "tenant_12345";

  const response: PostBanUserFromCommentResponse = await postBanUserFromComment(
    commentId,
    banEmail,
    undefined,
    banIP,
    deleteAllUsersComments,
    bannedUntil,
    isShadowBan,
    undefined,
    banReason,
    tenantId
  );
  console.log(response);
}
runBan();
[inline-code-end]