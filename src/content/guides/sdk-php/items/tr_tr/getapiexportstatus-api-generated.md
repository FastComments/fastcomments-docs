## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportStatusResponse.php)

## Örnek

[inline-code-attrs-start title = 'getApiExportStatus Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` arayüzünü uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$batch_job_id = 'batch_job_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getApiExportStatus($batch_job_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiExportStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]