## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Resposta

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo deleteQuestionResult'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
                // tratamento de sucesso
            }
            catch ( const std::exception& e )
            {
                // tratamento de erro
            }
        } );
}
[inline-code-end]