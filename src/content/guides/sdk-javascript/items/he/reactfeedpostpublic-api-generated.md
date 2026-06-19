---
## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| postId | string | כן |  |
| reactBodyParams | ReactBodyParams | כן |  |
| isUndo | boolean | לא |  |
| broadcastId | string | לא |  |
| sso | string | לא |  |

## תגובה

מוחזר: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-reactFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c9a';
const postId: string = 'post_97a4d2c1';
const reactBodyParams: ReactBodyParams = {
  reaction: 'heart',
  userId: 'user_42',
  timestamp: new Date().toISOString(),
  context: { device: 'mobile', appVersion: '5.3.1' }
};
const isUndo: boolean = false;
const broadcastId: string = 'broadcast_5f1b8';
const sso: string = 'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VySWQiOiJ1c2VyXzQyIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const result: ReactFeedPostResponse = await reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso);
[inline-code-end]

---