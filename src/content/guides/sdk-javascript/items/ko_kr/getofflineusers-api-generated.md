페이지에 과거에 댓글을 남긴 사용자로 현재 온라인 상태가 아닌 사람들입니다. displayName으로 정렬됩니다.
Use this after exhausting /users/online to render a "Members" section.
commenterName에 대한 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName} 인덱스를 afterName부터 $gt를 사용하여 앞으로 탐색하며 $skip 비용이 들지 않습니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니오 |  |
| afterUserId | string | 아니오 |  |

## 응답

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]