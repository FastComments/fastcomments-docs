현재 온라인 상태가 아닌, 해당 페이지의 과거 댓글 작성자들입니다. displayName 기준으로 정렬됩니다.
"멤버" 섹션을 렌더링하기 위해 /users/online을 모두 사용한 후에 이 엔드포인트를 사용하세요.
commenterName 기준 커서 페이지네이션: 서버는 afterName 이후부터 $gt를 사용하여 부분 {tenantId, urlId, commenterName}
인덱스를 순회하며, $skip 비용이 발생하지 않습니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니요 |  |
| afterUserId | string | 아니요 |  |

## 응답

반환값: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]