## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 예 |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | 예 |  |
| editKey | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## 예제

[inline-code-attrs-start title = 'setCommentText 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6721f4';
const commentId: string = 'cmt_9a3b2d';
const broadcastId: string = 'live_2026_03_25';
const editKey: string = 'edit_k_4f7b9';
const sso: string = 'sso_tok_eyJhbGciOiJIUzI1';
const commentTextUpdateRequest: CommentTextUpdateRequest = {
  text: 'Updated to clarify the timeline and link the relevant docs.',
  mentions: [{ userId: 'user_102', displayName: 'Alex Rivera' }],
  hashtags: [{ tag: 'product-update' }]
};
const result: SetCommentText200Response = await setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso);
[inline-code-end]

---