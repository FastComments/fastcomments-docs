Currently-online viewers of a page: 현재 웹소켓 세션이 해당 페이지에 구독 중인 사람들.

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).: anonCount + totalCount를 반환합니다 (방 전체 구독자 수, 열거되지 않은 익명 뷰어 포함).

## Parameters

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | const GetOnlineUsersOptions& | Yes |  |

## Response

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Example

[inline-code-attrs-start title = 'getOnlineUsers 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
auto options = std::make_shared<GetOnlineUsersOptions>();
options->maxResults = boost::optional<int>(100);
options->includeInactive = boost::optional<bool>(false);
api->getOnlineUsers(tenantId, urlId, *options).then([](pplx::task<std::shared_ptr<PageUsersOnlineResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]