Włącz lub wyłącz powiadomienia dla strony. Gdy użytkownicy są zapisani do strony, powiadomienia są tworzone
dla nowych komentarzy głównych, a także

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| urlId | string | query | Tak |  |
| url | string | query | Tak |  |
| pageTitle | string | query | Tak |  |
| subscribedOrUnsubscribed | string | path | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationPageSubscriptionStatusResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład updateUserNotificationPageSubscriptionStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta http, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To opcjonalne, `GuzzleHttp\Client` będzie używany domyślnie.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$url = 'url_example'; // string
$page_title = 'page_title_example'; // string
$subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example'; // string
$sso = 'sso_example'; // string


try {
    $result = $apiInstance->updateUserNotificationPageSubscriptionStatus($tenant_id, $url_id, $url, $page_title, $subscribed_or_unsubscribed, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationPageSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]