## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| value | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationSiteSearchResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getSearchSites'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se desejar usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional; `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchSites($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchSites: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]