租戶的使用者批次資訊。給定 userIds,從 User / SSOUser 回傳顯示資訊。
由留言小工具使用,以豐富透過 presence 事件剛出現的使用者。
無頁面上下文:隱私一律強制執行(私人個人檔案會被遮蔽)。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| ids | string | query | 是 | 以逗號分隔的 userIds。 |

## 回應

回傳:[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## 範例

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 http client,請傳入實作 `GuzzleHttp\ClientInterface` 的 client。
    // 這是選填,預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | 以逗號分隔的 userIds。


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]