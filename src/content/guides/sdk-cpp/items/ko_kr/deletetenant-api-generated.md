## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| sure | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'deleteTenant 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
boost::optional<utility::string_t> sure = utility::conversions::to_string_t("true");
api->deleteTenant(tenantId, userId, sure)
   .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
       try{
           auto resp = t.get();
           auto result = std::make_shared<APIEmptyResponse>(*resp);
       }catch(const std::exception&){})
[inline-code-end]