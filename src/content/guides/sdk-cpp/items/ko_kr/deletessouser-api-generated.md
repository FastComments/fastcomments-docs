## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| options | const DeleteSSOUserOptions& | 예 |  |

## 응답

반환: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSSOUserAPIResponse.h)

## 예시

[inline-code-attrs-start title = 'deleteSSOUser 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto userId = U("user-456");
DeleteSSOUserOptions options;
options.dryRun = boost::optional<bool>(true);
api->deleteSSOUser(tenantId, userId, options).then([](std::shared_ptr<DeleteSSOUserAPIResponse> resp) {
    if (resp) {
        (void)resp;
    }
});
[inline-code-end]