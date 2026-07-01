## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|------------|-----------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| fromName | string | query | Sim |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de sendInvite'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar autorização da chave API: api_key
// Descomente abaixo para definir o prefixo (ex.: Bearer) da chave API, se necessário
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se desejar usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$from_name = 'from_name_example'; // string


try {
    $result = $apiInstance->sendInvite($tenant_id, $id, $from_name);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->sendInvite: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]