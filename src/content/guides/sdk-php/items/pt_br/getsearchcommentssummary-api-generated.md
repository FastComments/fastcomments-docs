## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| value | string | query | Não |  |
| filters | string | query | Não |  |
| searchFilters | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationCommentSearchResponse.php)

## Exemplo

[inline-code-attrs-start title = 'getSearchCommentsSummary Exemplo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\ModerationApi(
//     // Se você quiser usar um cliente http personalizado, passe seu cliente que implementa `GuzzleHttp\ClientInterface`.
//     // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
//     new GuzzleHttp\Client()
// );

$tenant_id = 'tenant_id_example'; // string
$options = [
    'value' => 'value_example', // string
    'filters' => 'filters_example', // string
    'search_filters' => 'search_filters_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getSearchCommentsSummary($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchCommentsSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]