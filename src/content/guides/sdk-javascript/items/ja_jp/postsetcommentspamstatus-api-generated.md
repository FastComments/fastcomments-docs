## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | はい |  |
| spam | boolean | いいえ |  |
| permNotSpam | boolean | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'postSetCommentSpamStatus の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = 'cmt_9f8b3a2e';
const spam: boolean = false;
const permNotSpam: boolean = true;
const sso: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signedToken';
const result: APIEmptyResponse = await postSetCommentSpamStatus(commentId, spam, permNotSpam, sso);
[inline-code-end]

---