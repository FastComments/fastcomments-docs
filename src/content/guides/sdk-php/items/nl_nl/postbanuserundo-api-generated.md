## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| sso | string | query | Nee |  |

## Respons

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'postBanUserUndo Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client op die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt als standaard gebruikt.
    new GuzzleHttp\Client()
);
$ban_user_undo_params = new \FastComments\Client\Model\BanUserUndoParams(); // \FastComments\Client\Model\BanUserUndoParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postBanUserUndo($ban_user_undo_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserUndo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]