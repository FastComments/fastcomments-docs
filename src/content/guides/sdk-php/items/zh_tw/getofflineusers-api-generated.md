在該頁面上，但目前不在線上的過去評論者。依 displayName 排序。
在用完 /users/online 之後使用此方法以呈現「成員」區段。
在 commenterName 上使用游標分頁：伺服器會在部分索引 {tenantId, urlId, commenterName}
從 afterName 向前透過 $gt 遍歷，無需 $skip 成本。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別碼（伺服器端已清理）。 |
| afterName | string | query | No | 游標：從先前的回應傳遞 nextAfterName。 |
| afterUserId | string | query | No | 游標 決勝者：從先前的回應傳遞 nextAfterUserId。當設定 afterName 時為必要，以免名稱相同導致條目遺失。 |

## 回應

回傳: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 客戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 頁面 URL 識別碼（伺服器端已清理）。
$after_name = 'after_name_example'; // string | 游標：從先前的回應傳遞 nextAfterName。
$after_user_id = 'after_user_id_example'; // string | 游標 決勝者：從先前的回應傳遞 nextAfterUserId。當設定 afterName 時為必要，以免名稱相同導致條目遺失。

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]