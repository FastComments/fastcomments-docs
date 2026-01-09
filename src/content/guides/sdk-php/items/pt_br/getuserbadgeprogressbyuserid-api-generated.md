## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| userId | string | path | Sim |  |

## Resposta

Retorna: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadgeProgressById200Response.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserBadgeProgressByUserId'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar autorização da chave da API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomente abaixo para configurar prefixo (por exemplo Bearer) para a chave da API, se necessário
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional; `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->getUserBadgeProgressByUserId($tenant_id, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressByUserId: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]