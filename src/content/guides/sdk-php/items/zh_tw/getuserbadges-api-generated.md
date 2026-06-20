## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| badgeId | string | query | 否 |  |
| type | number | query | 否 |  |
| displayedOnComments | boolean | query | 否 |  |
| limit | number | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgesResponse.php)

## 範例

[inline-code-attrs-start title = 'getUserBadges 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如需，解除註解下方以設定 API 金鑰的前綴（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 若要使用自訂的 http 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，`GuzzleHttp\Client` 將作為預設。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$badge_id = 'badge_id_example'; // string
$type = 3.4; // float
$displayed_on_comments = True; // bool
$limit = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getUserBadges($tenant_id, $user_id, $badge_id, $type, $displayed_on_comments, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---