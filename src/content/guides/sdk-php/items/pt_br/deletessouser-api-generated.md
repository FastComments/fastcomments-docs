## Parâmetros

| Nome | Type | Location | Obrigatório | Descrição |
|------|------|----------|------------|-----------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| deleteComments | boolean | query | Não |  |
| commentDeleteMode | string | query | Não |  |

## Resposta

Retorna: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSSOUserAPIResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteSSOUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar autorização da chave de API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomente abaixo para configurar prefixo (por exemplo, Bearer) para a chave de API, se necessário
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado por padrão.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$delete_comments = True; // bool
$comment_delete_mode = 'comment_delete_mode_example'; // string

try {
    $result = $apiInstance->deleteSSOUser($tenant_id, $id, $delete_comments, $comment_delete_mode);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]