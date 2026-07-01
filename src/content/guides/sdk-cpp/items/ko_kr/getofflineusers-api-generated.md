Past commenters on the page who are NOT currently online. Sorted by displayName.  
현재 온라인이 아닌 페이지의 과거 댓글자들. displayName 기준으로 정렬됩니다.

Use this after exhausting /users/online to render a "Members" section.  
/users/online을 사용한 후에, "Members" 섹션을 렌더링하기 위해 사용합니다.

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
commenterName에 대한 커서 페이지네이션: 서버는 {tenantId, urlId, commenterName} 부분 인덱스를 afterName 이후로 $gt를 사용해 진행하며, $skip 비용이 없습니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | const GetOfflineUsersOptions& | Yes |  |

## 응답

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

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