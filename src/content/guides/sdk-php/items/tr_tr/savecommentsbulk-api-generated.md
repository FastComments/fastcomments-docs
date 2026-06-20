## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| isLive | boolean | query | Hayır |  |
| doSpamCheck | boolean | query | Hayır |  |
| sendEmails | boolean | query | Hayır |  |
| populateNotifications | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SaveCommentsBulkResponse.php)

## Örnek

[inline-code-attrs-start title = 'saveCommentsBulk Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// API anahtarı yetkilendirmesini yapılandır: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak üzere aşağıdaki satırın yorumunu kaldırın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçin.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_comment_params = array(new \FastComments\Client\Model\CreateCommentParams()); // \FastComments\Client\Model\CreateCommentParams[]
$is_live = True; // bool
$do_spam_check = True; // bool
$send_emails = True; // bool
$populate_notifications = True; // bool

try {
    $result = $apiInstance->saveCommentsBulk($tenant_id, $create_comment_params, $is_live, $do_spam_check, $send_emails, $populate_notifications);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->saveCommentsBulk: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]