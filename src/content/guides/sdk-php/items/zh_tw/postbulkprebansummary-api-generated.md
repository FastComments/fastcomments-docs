## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## 回傳

回傳: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkPreBanSummary.php)

## 範例

[inline-code-attrs-start title = 'postBulkPreBanSummary 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 若要使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
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

---