## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|------|-------------|-----------|
| tenantId | string | path | Sim |  |
| postIds | array | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UserReactsResponse.php)

## Exemplo

[inline-code-attrs-start title = 'getUserReactsPublic Exemplo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implementa `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'post_ids' => array('post_ids_example'), // string[]
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getUserReactsPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserReactsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---