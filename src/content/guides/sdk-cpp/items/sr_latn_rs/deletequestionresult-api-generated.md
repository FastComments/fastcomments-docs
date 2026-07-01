## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Primer

[inline-code-attrs-start title = 'deleteQuestionResult Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
                // uspešno rukovanje
            }
            catch ( const std::exception& e )
            {
                // obrada greške
            }
        } );
}
[inline-code-end]