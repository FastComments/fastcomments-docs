페이지에서 현재 온라인 상태가 아닌 이전 댓글 작성자입니다. displayName으로 정렬됩니다.
이것은 /users/online을 모두 사용한 후 'Members' 섹션을 렌더링하기 위해 사용하세요.
commenterName에 대한 커서 기반 페이징: 서버는 부분 인덱스 {tenantId, urlId, commenterName}를 사용하여 afterName 이후부터 $gt로 앞으로 순회하며 $skip 비용이 발생하지 않습니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니요 |  |
| afterUserId | string | 아니요 |  |

## 응답

반환: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]