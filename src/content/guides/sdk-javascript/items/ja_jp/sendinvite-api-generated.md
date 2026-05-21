## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| fromName | string | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'sendInvite の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-128';
const id: string = 'comment-8421f';
const fromName: string = 'Marcus Lindström';
const note: string | undefined = undefined; // オプションのパラメータ例
const response: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName);
[inline-code-end]

---