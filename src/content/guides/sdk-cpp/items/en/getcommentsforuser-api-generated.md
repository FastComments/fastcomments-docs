## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| options | const GetCommentsForUserOptions& | Yes |  |

## Response

Returns: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsForUserResponse.h)

## Example

[inline-code-attrs-start title = 'getCommentsForUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetCommentsForUserOptions{
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("user@example.com"),
    boost::optional<int>(50),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("next-page-token"))
};

api->getCommentsForUser(options).then([](pplx::task<std::shared_ptr<GetCommentsForUserResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){}
});
[inline-code-end]
