## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vrne: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UnblockSuccess.php)

## Primer

[inline-code-attrs-start title = 'unBlockUserFromComment Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte avtorizacijo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodnjo vrstico za nastavitev prefiksa (npr. Bearer) za API ključ, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti prilagojen HTTP odjemalec, posredujte vaš odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je izbirno, privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$un_block_from_comment_params = new \FastComments\Client\Model\UnBlockFromCommentParams(); // \FastComments\Client\Model\UnBlockFromCommentParams
$user_id = 'user_id_example'; // string
$anon_user_id = 'anon_user_id_example'; // string

try {
    $result = $apiInstance->unBlockUserFromComment($tenant_id, $id, $un_block_from_comment_params, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->unBlockUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]