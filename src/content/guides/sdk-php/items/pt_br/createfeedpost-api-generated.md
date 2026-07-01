## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|------------|
| tenantId | string | query | Sim |  |
| broadcastId | string | query | Não |  |
| isLive | boolean | query | Não |  |
| doSpamCheck | boolean | query | Não |  |
| skipDupCheck | boolean | query | Não |  |

## Resposta

Retorna: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPostsResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo createFeedPost'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura a autorização da chave API: api_key
// Descomente abaixo para definir o prefixo (ex.: Bearer) para a chave API, se necessário
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implementa `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$create_feed_post_params = new \FastComments\Client\Model\CreateFeedPostParams(); // \FastComments\Client\Model\CreateFeedPostParams
$options = [
    'broadcast_id' => 'broadcast_id_example', // string
    'is_live' => True, // bool
    'do_spam_check' => True, // bool
    'skip_dup_check' => True, // bool
];


try {
    $result = $apiInstance->createFeedPost($tenant_id, $create_feed_post_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createFeedPost: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]