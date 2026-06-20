## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| userId | string | query | Não |  |
| limit | number | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgeProgressListResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserBadgeProgressList'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure a autorização da chave da API: api_key
// Descomente abaixo para configurar o prefixo (ex.: Bearer) para a chave da API, se necessário
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$limit = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getUserBadgeProgressList($tenant_id, $user_id, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressList: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---