페이지의 현재 온라인 뷰어: 웹소켓 세션이 현재 해당 페이지를 구독 중인 사람들.
anonCount + totalCount을 반환합니다 (룸 전체 구독자, 우리가 개별적으로 열거하지 않는 익명 뷰어 포함).

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니오 |  |
| afterUserId | string | 아니오 |  |

## 응답

반환: [`GetOnlineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsers200Response.ts)

## 예제

[inline-code-attrs-start title = 'getOnlineUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_14f9c3';
const urlId: string = 'article_20250615';
const afterName: string = 'marie.curie';
const afterUserId: string = 'u_92b7';
const result: GetOnlineUsers200Response = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---