## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## 响应

返回: [`PinCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinCommentResponse.ts)

## 示例

[inline-code-attrs-start title = 'pinComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant-001';
  const commentId: string = 'comment-5678';
  const broadcastId: string = 'broadcast-2023';
  const ssoToken: string = 'sso-xyz-789';

  const pinResult: PinCommentResponse = await pinComment(tenantId, commentId, broadcastId);
  const pinResultWithSso: PinCommentResponse = await pinComment(tenantId, commentId, broadcastId, ssoToken);
})();
[inline-code-end]