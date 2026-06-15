## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updatableCommentParams | UpdatableCommentParams | Evet |  |
| contextUserId | string | Hayır |  |
| doSpamCheck | boolean | Hayır |  |
| isLive | boolean | Hayır |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Örnek

[inline-code-attrs-start title = 'updateComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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