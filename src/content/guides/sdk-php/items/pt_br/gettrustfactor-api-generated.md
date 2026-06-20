## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| userId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserTrustFactorResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTrustFactor'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional; `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getTrustFactor($user_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]