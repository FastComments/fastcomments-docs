## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postId | string | 예 |  |
| reactBodyParams | ReactBodyParams | 예 |  |
| isUndo | boolean | 아니요 |  |
| broadcastId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'reactFeedPostPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'd290f1ee-6c54-4b01-90e6-d701748f0851';
const postId: string = 'c1a2b3d4-5678-90ab-cdef-1234567890ab';
const reactBodyParams: ReactBodyParams = { reaction: 'like', emoji: '👍', source: 'web' };
const isUndo: boolean = false;
const broadcastId: string = 'broadcast-2026-03-25-001';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const result: ReactFeedPostPublic200Response = await reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso);
[inline-code-end]

---