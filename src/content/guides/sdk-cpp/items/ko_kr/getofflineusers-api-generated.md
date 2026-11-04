현재 온라인 상태가 아닌, 해당 페이지의 과거 댓글 작성자입니다. displayName 기준으로 정렬됩니다.
"Members" 섹션을 렌더링할 때 /users/online을 모두 소진한 후 이 API를 사용하세요.
commenterName 기준 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName}
인덱스를 afterName부터 $gt로 순회하며, $skip 비용이 없습니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| options | const GetOfflineUsersOptions& | 예 |  |

## 응답

반환값: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]