## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| text-search | string | query | Não |  |
| byIPFromComment | string | query | Não |  |
| filters | string | query | Não |  |
| searchFilters | string | query | Não |  |
| afterId | string | query | Não |  |
| demo | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentIdsResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getApiIds'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // string
$by_ip_from_comment = 'by_ip_from_comment_example'; // string
$filters = 'filters_example'; // string
$search_filters = 'search_filters_example'; // string
$after_id = 'after_id_example'; // string
$demo = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getApiIds($text_search, $by_ip_from_comment, $filters, $search_filters, $after_id, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiIds: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]