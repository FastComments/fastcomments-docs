## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostRemoveCommentResponse.ts)

## 示例

[inline-code-attrs-start title = 'postRemoveComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId1: string = '6f8b0c2e-9f8e-4a3c-9dfa-2b1f6e7a3c9d';
const removed1: PostRemoveCommentResponse = await postRemoveComment(commentId1);

const commentId2: string = 'b7e1f4a2-3d9c-4cfa-8b1e-5d6a2f9b8c7e';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NjciLCJlbWFpbCI6InVzZXJAZXhhbXBsZS5jb20ifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const removed2: PostRemoveCommentResponse = await postRemoveComment(commentId2, ssoToken);
[inline-code-end]

---