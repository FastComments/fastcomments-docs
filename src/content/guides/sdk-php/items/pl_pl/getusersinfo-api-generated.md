Zbiorcze informacje o użytkownikach dla tenanta. Na podstawie podanych userIds zwraca informacje wyświetlane z User / SSOUser.
Używane przez widget komentarzy do wzbogacania użytkowników, którzy właśnie pojawili się w wyniku zdarzenia obecności.
Brak kontekstu strony: prywatność jest stosowana jednakowo (prywatne profile są maskowane).

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| ids | string | query | Tak | Lista userIds rozdzielonych przecinkami. |

## Odpowiedź

Zwraca: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne — domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | userIds rozdzielone przecinkami.

try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]