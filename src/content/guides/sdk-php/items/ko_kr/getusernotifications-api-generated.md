## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 아니오 | 현재 페이지가 구독되어 있는지 여부를 판단하는 데 사용됩니다. |
| pageSize | integer | query | 아니오 |  |
| afterId | string | query | 아니오 |  |
| includeContext | boolean | query | 아니오 |  |
| afterCreatedAt | integer | query | 아니오 |  |
| unreadOnly | boolean | query | 아니오 |  |
| dmOnly | boolean | query | 아니오 |  |
| noDm | boolean | query | 아니오 |  |
| includeTranslations | boolean | query | 아니오 |  |
| includeTenantNotifications | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetMyNotificationsResponse.php)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항이며 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$url_id = 'url_id_example'; // 문자열 | 현재 페이지가 구독되어 있는지 여부를 판단하는 데 사용됩니다.
$page_size = 56; // 정수
$after_id = 'after_id_example'; // 문자열
$include_context = True; // 부울
$after_created_at = 56; // 정수
$unread_only = True; // 부울
$dm_only = True; // 부울
$no_dm = True; // 부울
$include_translations = True; // 부울
$include_tenant_notifications = True; // 부울
$sso = 'sso_example'; // 문자열

try {
    $result = $apiInstance->getUserNotifications($tenant_id, $url_id, $page_size, $after_id, $include_context, $after_created_at, $unread_only, $dm_only, $no_dm, $include_translations, $include_tenant_notifications, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]