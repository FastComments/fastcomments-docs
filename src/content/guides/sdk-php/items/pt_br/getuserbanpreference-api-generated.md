## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Não |  |

## Resposta

Retorna: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIModerateGetUserBanPreferencesResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserBanPreference'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado por padrão.
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserBanPreference($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getUserBanPreference: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---