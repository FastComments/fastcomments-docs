테넌트의 페이지 목록을 나열합니다. FChat 데스크탑 클라이언트에서 방 목록을 채우는 데 사용됩니다.
각 페이지에 대해 확정된 사용자 정의 구성에서 `enableFChat`이 true여야 합니다.
SSO를 요구하는 페이지는 요청한 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| cursor | string | 아니오 |  |
| limit | int32_t | 아니오 |  |
| q | string | 아니오 |  |
| sortBy | PagesSortBy | 아니오 |  |
| hasComments | bool | 아니오 |  |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## 예제

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> cursor = utility::string_t(U("cursor_abc"));
boost::optional<int32_t> limit = 50;
boost::optional<utility::string_t> q = utility::string_t(U("status:published"));
boost::optional<PagesSortBy> sortBy = PagesSortBy::NEWEST;
boost::optional<bool> hasComments = true;
api->getPagesPublic(tenantId, cursor, limit, q, sortBy, hasComments)
.then([](std::shared_ptr<GetPublicPagesResponse> resp){
    if (!resp) resp = std::make_shared<GetPublicPagesResponse>();
})
.wait();
[inline-code-end]

---