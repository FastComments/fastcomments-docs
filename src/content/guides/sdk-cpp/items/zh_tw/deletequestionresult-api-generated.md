## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 回應

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'deleteQuestionResult 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> optTenant = utility::conversions::to_string_t( "my-tenant-123" );
utility::string_t questionId = utility::conversions::to_string_t( "question-456" );

if ( optTenant )
{
    api->deleteQuestionResult( *optTenant, questionId )
        .then( []( pplx::task<std::shared_ptr<APIEmptyResponse>> t )
        {
            try
            {
                auto resp = t.get();
                // 成功處理
            }
            catch ( const std::exception& e )
            {
                // 錯誤處理
            }
        } );
}
[inline-code-end]