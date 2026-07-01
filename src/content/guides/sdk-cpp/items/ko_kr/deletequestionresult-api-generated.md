## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## Response

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'deleteQuestionResult 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
                // 성공 처리
            }
            catch ( const std::exception& e )
            {
                // 오류 처리
            }
        } );
}
[inline-code-end]