## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| contextUserId | string | query | Não |  |
| doSpamCheck | boolean | query | Não |  |
| isLive | boolean | query | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo updateComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurar autorização de chave de API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Descomente abaixo para configurar prefixo (ex.: Bearer) para a chave de API, se necessário
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se quiser usar cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$updatable_comment_params = new \FastComments\Client\Model\UpdatableCommentParams(); // \FastComments\Client\Model\UpdatableCommentParams
$options = [
    'context_user_id' => 'context_user_id_example', // string
    'do_spam_check' => True, // bool
    'is_live' => True, // bool
];


try {
    $result = $apiInstance->updateComment($tenant_id, $id, $updatable_comment_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]