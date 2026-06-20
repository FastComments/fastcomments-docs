## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| value | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationUserSearchResponse.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getSearchUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Isso é opcional, `GuzzleHttp\Client` será usado como padrão.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchUsers($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---