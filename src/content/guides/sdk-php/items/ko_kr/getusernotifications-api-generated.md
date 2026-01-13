## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| pageSize | integer | query | 아니오 |  |
| afterId | string | query | 아니오 |  |
| includeContext | boolean | query | 아니오 |  |
| afterCreatedAt | integer | query | 아니오 |  |
| unreadOnly | boolean | query | 아니오 |  |
| dmOnly | boolean | query | 아니오 |  |
| noDm | boolean | query | 아니오 |  |
| includeTranslations | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserNotifications200Response.php)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항입니다. 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$page_size = 56; // 정수
$after_id = 'after_id_example'; // 문자열
$include_context = True; // 불리언
$after_created_at = 56; // 정수
$unread_only = True; // 불리언
$dm_only = True; // 불리언
$no_dm = True; // 불리언
$include_translations = True; // 불리언
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->getUserNotifications($tenant_id, $page_size, $after_id, $include_context, $after_created_at, $unread_only, $dm_only, $no_dm, $include_translations, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---