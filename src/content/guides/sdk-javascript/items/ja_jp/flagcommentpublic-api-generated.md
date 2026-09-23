## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| isFlagged | boolean | はい |  |
| sso | string | いいえ |  |

## 応答

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'flagCommentPublic 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const commentId: string = "comment_1001";
const isFlagged: boolean = true;
const sso: string = "sso_user_5678";

const resultWithSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, isFlagged, sso);
const resultWithoutSso: APIEmptyResponse = await flagCommentPublic(tenantId, commentId, false);
[inline-code-end]