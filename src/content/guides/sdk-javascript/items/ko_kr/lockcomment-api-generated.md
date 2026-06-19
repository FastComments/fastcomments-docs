## 파라미터

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 예제

[inline-code-attrs-start title = 'lockComment 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media-214';
const commentId: string = 'cmt_4f3b9a2d';
const broadcastId: string = 'live-987654321';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NzgiLCJuYW1lIjoiSmFuZSBEb2UifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

const lockedWithSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId, sso);
const lockedWithoutSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId);
[inline-code-end]

---