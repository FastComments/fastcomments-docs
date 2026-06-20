## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| largeInternalURLSanitized | string | 是 |  |

## 响应

返回：[`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GifGetLargeResponse.h)

## 示例

[inline-code-attrs-start title = 'getGifLarge 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t largeUrl = U("https://cdn.fastcomments.com/gifs/large/abc123.gif");
boost::optional<utility::string_t> acceptLanguage = boost::optional<utility::string_t>(U("en-US"));
api->getGifLarge(tenantId, largeUrl)
    .then([=](pplx::task<std::shared_ptr<GifGetLargeResponse>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                std::cout << "GIF retrieved successfully\n";
            } else {
                auto fallback = std::make_shared<GifGetLargeResponse>();
                std::cout << "Empty response, using fallback\n";
            }
        } catch (const std::exception &e) {
            std::cout << "Request failed: " << e.what() << '\n';
        }
    }).wait();
[inline-code-end]