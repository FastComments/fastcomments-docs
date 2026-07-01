## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Beispiel

[inline-code-attrs-start title = 'deleteQuestionResult Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
                // Erfolgsbehandlung
            }
            catch ( const std::exception& e )
            {
                // Fehlerbehandlung
            }
        } );
}
[inline-code-end]