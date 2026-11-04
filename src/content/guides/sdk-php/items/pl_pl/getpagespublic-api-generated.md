Wyświetla listę stron dla tenanta. Używane przez klienta desktopowego FChat do wypełnienia listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozwiązanej niestandardowej konfiguracji dla każdej strony.
Strony wymagające SSO są filtrowane względem dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| cursor | string | query | Nie | Nieprzezroczysty kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | Nie | 1..200, domyślnie 50 |
| q | string | query | Nie | Opcjonalny filtr prefiksu tytułu, nieuwzględniający wielkości liter. |
| sortBy | string | query | Nie | Kolejność sortowania. `updatedAt` (domyślna, od najnowszych), `commentCount` (najpierw najwięcej komentarzy) lub `title` (alfabetycznie). |
| hasComments | boolean | query | Nie | Jeśli true, zwraca tylko strony z co najmniej jednym komentarzem. |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta http, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To opcjonalne, `GuzzleHttp\Client` będzie używany domyślnie.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Nieprzezroczysty kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`.
    'limit' => 56, // int | 1..200, domyślnie 50
    'q' => 'q_example', // string | Opcjonalny filtr prefiksu tytułu, nieuwzględniający wielkości liter.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Kolejność sortowania. `updatedAt` (domyślna, od najnowszych), `commentCount` (najpierw najwięcej komentarzy) lub `title` (alfabetycznie).
    'has_comments' => True, // bool | Jeśli true, zwraca tylko strony z co najmniej jednym komentarzem.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]