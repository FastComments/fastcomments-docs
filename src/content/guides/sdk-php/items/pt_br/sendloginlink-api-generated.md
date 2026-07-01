## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| redirectURL | string | query | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo sendLoginLink'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Configura a autorização da chave de API: api_key
// Descomente abaixo para definir o prefixo (ex.: Bearer) para a chave de API, se necessário
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$redirect_url = 'redirect_url_example'; // string


try {
    $result = $apiInstance->sendLoginLink($tenant_id, $id, $redirect_url);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->sendLoginLink: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---