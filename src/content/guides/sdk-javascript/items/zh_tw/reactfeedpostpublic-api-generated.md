## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| reactBodyParams | ReactBodyParams | Yes |  |
| isUndo | boolean | No |  |
| broadcastId | string | No |  |
| urlId | string | No |  |
| sso | string | No |  |

## 回應

回傳: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'reactFeedPostPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f2b1';
const postId: string = 'post_12ac9e';
const reactBodyParams: ReactBodyParams = { emoji: 'thumbs_up', intensity: 1 };
const isUndo: boolean = false;
const broadcastId: string = 'broadcast_20260503_01';
const urlId: string = 'article-4527';
const sso: string = 'sso_token_7f3b2c';

const result: ReactFeedPostPublic200Response = await reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, urlId, sso);
[inline-code-end]

---