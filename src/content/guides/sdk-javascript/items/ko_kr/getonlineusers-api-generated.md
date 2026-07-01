현재 페이지에 실시간으로 접속 중인 사용자: 현재 웹소켓 세션이 페이지에 구독돼 있는 사람들입니다.  
익명 사용자 수(anonCount)와 총 사용자 수(totalCount)를 반환합니다(방 전체 구독자 수, 열거되지 않은 익명 뷰어 포함).

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니오 |  |
| afterUserId | string | 아니오 |  |

## 응답

반환: [`GetOnlineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsersResponse.ts)

## 예시

[inline-code-attrs-start title = 'getOnlineUsers 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_98765";

  // 선택적 페이지 매김 매개변수가 있는 경우
  const pagedResult: GetOnlineUsersResponse = await getOnlineUsers(
    tenantId,
    urlId,
    "alice_smith",
    "user_9"
  );

  // 선택적 페이지 매김 매개변수가 없는 경우
  const fullResult: GetOnlineUsersResponse = await getOnlineUsers(tenantId, urlId);
}
[inline-code-end]