현재 페이지에 온라인으로 보는 사람: 현재 웹소켓 세션이 해당 페이지에 구독된 사람들.
익명 카운트와 전체 카운트(방 전체 구독자, 열거하지 않는 익명 뷰어 포함)를 반환합니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니오 |  |
| afterUserId | string | 아니오 |  |

## 응답

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## 예제

[inline-code-attrs-start title = 'getOnlineUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "article-9876";
  const afterName: string | undefined = "john_doe";
  const afterUserId: string | undefined = "user_456";

  const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(onlineUsers);
}

fetchOnlineUsers();
[inline-code-end]

---