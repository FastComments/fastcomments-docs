---
## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updatableCommentParams | UpdatableCommentParams | 是 |  |
| contextUserId | string | 否 |  |
| doSpamCheck | boolean | 否 |  |
| isLive | boolean | 否 |  |

## 响应

返回：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'updateComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3f47b2a1";
const id: string = "comment_9a12b3c4";
const updatableCommentParams: UpdatableCommentParams = {
  body: "Thanks for the update — I've adjusted my view accordingly."
};
const contextUserId: string = "user_8721";
const doSpamCheck: boolean = true;
const isLive: boolean = false;
const result: FlagCommentPublic200Response = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]

---