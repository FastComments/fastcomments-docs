## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Не |  |
| includeByIP | boolean | query | Не |  |
| includeByEmailDomain | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkPreBanSummary.php)

## Примјер

[inline-code-attrs-start title = 'postBulkPreBanSummary Примјер'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће се користити `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$bulk_pre_ban_params = new \FastComments\Client\Model\BulkPreBanParams(); // \FastComments\Client\Model\BulkPreBanParams
$include_by_user_id_and_email = True; // bool
$include_by_ip = True; // bool
$include_by_email_domain = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postBulkPreBanSummary($bulk_pre_ban_params, $include_by_user_id_and_email, $include_by_ip, $include_by_email_domain, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBulkPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]