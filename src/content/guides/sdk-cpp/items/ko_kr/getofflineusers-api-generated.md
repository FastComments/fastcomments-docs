현재 온라인 상태가 아닌 페이지의 이전 댓글 작성자들입니다. displayName으로 정렬됩니다.
"/users/online"을 모두 소진한 후 "Members" 섹션을 렌더링하기 위해 사용하세요.
commenterName에 대한 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName}
인덱스를 afterName에서 앞으로 $gt를 통해 순회하며, $skip 비용이 없습니다.

## 매개변수

| Name | Type | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| afterName | string | 아니오 |  |
| afterUserId | string | 아니오 |  |

## 응답

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> afterName = boost::optional<utility::string_t>(U("jane.doe@example.com"));
boost::optional<utility::string_t> afterUserId = boost::optional<utility::string_t>(U("user-789"));
api->getOfflineUsers(tenantId, urlId, afterName, afterUserId).then([](std::shared_ptr<PageUsersOfflineResponse> resp){
    auto result = resp ? resp : std::make_shared<PageUsersOfflineResponse>();
    (void)result;
});
[inline-code-end]

---