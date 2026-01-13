## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| deleteComments | string | query | Não |  |
| commentDeleteMode | string | query | Não |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteTenantUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar autorização da chave da API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomente abaixo para configurar o prefixo (por exemplo Bearer) para a chave da API, se necessário
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se você quiser usar um cliente HTTP customizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado por padrão.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$delete_comments = 'delete_comments_example'; // string
$comment_delete_mode = 'comment_delete_mode_example'; // string

try {
    $result = $apiInstance->deleteTenantUser($tenant_id, $id, $delete_comments, $comment_delete_mode);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]