## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| postId | string | 是 |  |
| reactBodyParams | ReactBodyParams | 是 |  |
| isUndo | boolean | 否 |  |
| broadcastId | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostResponse.ts)

## 範例

[inline-code-attrs-start title = 'reactFeedPostPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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