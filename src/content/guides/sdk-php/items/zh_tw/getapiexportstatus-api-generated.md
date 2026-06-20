## 參數

| 名稱 | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| batchJobId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportStatusResponse.php)

## 範例

[inline-code-attrs-start title = 'getApiExportStatus 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自訂的 HTTP 客戶端，傳入實作了 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$batch_job_id = 'batch_job_id_example'; // 字串
$sso = 'sso_example'; // 字串

try {
    $result = $apiInstance->getApiExportStatus($batch_job_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiExportStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---