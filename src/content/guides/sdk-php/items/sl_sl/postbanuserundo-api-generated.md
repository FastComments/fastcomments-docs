## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| sso | string | poizvedba | Ne |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer postBanUserUndo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Če želite uporabiti lasten HTTP odjemalec, posredujte odjemalca, ki uresničuje `GuzzleHttp\ClientInterface`.
    // To je neobvezno, kot privzeti bo uporabljen `GuzzleHttp\Client`.
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