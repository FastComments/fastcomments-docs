---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updatableCommentParams | UpdatableCommentParams | 是 |  |
| contextUserId | string | 否 |  |
| doSpamCheck | boolean | 否 |  |
| isLive | boolean | 否 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'updateComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3c1b2a';
const commentId: string = 'cmt_8d9f2a4b';
const updatableCommentParams: UpdatableCommentParams = {
  body: 'Updating this comment to clarify the feature behavior and include a timestamp.',
  metadata: { category: 'support', editedReason: 'clarify instructions' },
  visible: true
};
const contextUserId: string = 'user_42';
const doSpamCheck: boolean = true;
const result: FlagCommentPublic200Response = await updateComment(tenantId, commentId, updatableCommentParams, contextUserId, doSpamCheck);
[inline-code-end]

---