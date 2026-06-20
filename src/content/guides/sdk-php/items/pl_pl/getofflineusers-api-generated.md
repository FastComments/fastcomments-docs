Poprzedni komentujący na stronie, którzy NIE są aktualnie online. Posortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyrenderować sekcję "Members".
Paginacja kursorowa po commenterName: serwer przegląda częściowy indeks {tenantId, urlId, commenterName} od afterName w przód za pomocą $gt, bez kosztu $skip.

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (czyszczony po stronie serwera). |
| afterName | string | query | No | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby powiązania nazw nie powodowały pominięcia wpisów. |

## Odpowiedź

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta implementującego `GuzzleHttp\ClientInterface`.
    // To opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identyfikator URL strony (czyszczony po stronie serwera).
$after_name = 'after_name_example'; // string | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi.
$after_user_id = 'after_user_id_example'; // string | Rozstrzygacz kursora: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane gdy afterName jest ustawione, aby powiązania nazw nie powodowały pominięcia wpisów.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]