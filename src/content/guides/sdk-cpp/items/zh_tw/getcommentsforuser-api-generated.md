## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| options | const GetCommentsForUserOptions& | 是 |  |

## 回應

返回：[`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsForUserResponse.h)

## 範例

[inline-code-attrs-start title = 'getCommentsForUser 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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