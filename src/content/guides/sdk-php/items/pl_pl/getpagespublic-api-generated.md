Lista stron dla tenanta. Używane przez klienta desktopowego FChat do wypełnienia jego listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozwiązanej konfiguracji niestandardowej dla każdej strony.
Strony wymagające SSO są filtrowane na podstawie dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| cursor | string | query | Nie | Niejawny kursor paginacji zwrócony jako `nextCursor` z wcześniejszego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | Nie | 1..200, domyślnie 50 |
| q | string | query | Nie | Opcjonalny filtr prefiksu tytułu niewrażliwy na wielkość liter. |
| sortBy | string | query | Nie | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze pierwsze), `commentCount` (najwięcej komentarzy pierwsze), lub `title` (alfabetycznie). |
| hasComments | boolean | query | Nie | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem. |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, jako domyślny zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Niejawny kursor paginacji zwrócony jako `nextCursor` z wcześniejszego żądania. Powiązany z tym samym `sortBy`.
$limit = 56; // int | 1..200, domyślnie 50
$q = 'q_example'; // string | Opcjonalny filtr prefiksu tytułu niewrażliwy na wielkość liter.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze pierwsze), `commentCount` (najwięcej komentarzy pierwsze), lub `title` (alfabetycznie).
$has_comments = True; // bool | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]