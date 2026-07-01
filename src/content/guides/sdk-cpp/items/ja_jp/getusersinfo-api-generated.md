Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
テナントのユーザー情報を一括取得します。userIds が指定されると、User / SSOUser から表示情報を返します。

Used by the comment widget to enrich users that just appeared via a presence event.  
コメントウィジェットが、プレゼンスイベントで新たに現れたユーザーを補強するために使用されます。

No page context: privacy is enforced uniformly (private profiles are masked).  
ページコンテキストなし: プライバシーは一様に適用され、プライベートプロファイルはマスクされます。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Example

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // レスポンスを処理
    }catch(const std::exception&){
        // エラーを処理
    }
});
[inline-code-end]