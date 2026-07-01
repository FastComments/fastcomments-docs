req
tenantId
urlId
userIdWS

## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userIdWS | string | 是 |  |
| startTime | int64_t | 是 |  |
| endTime | int64_t | 否 |  |

## 回應

返回：[`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## 範例

[inline-code-attrs-start title = 'getGlobalEventLog 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->getGlobalEventLog(
    U("my-tenant-123"),
    U("article-456"),
    U("user@example.com"),
    1622505600,
    boost::optional<int64_t>(1625097600)
).then([](std::shared_ptr<GetEventLogResponse> resp) {
    (void)resp;
});
[inline-code-end]

---