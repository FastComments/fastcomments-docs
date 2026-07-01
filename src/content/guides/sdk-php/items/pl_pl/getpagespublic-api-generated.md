List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Niewidoczny wskaźnik stronicowania zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | No | 1..200, domyślnie 50 |
| q | string | query | No | Opcjonalny filtr prefiksu tytułu, nie rozróżniający wielkości liter. |
| sortBy | string | query | No | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze najpierw), `commentCount` (najwięcej komentarzy najpierw) lub `title` (alfabetycznie). |
| hasComments | boolean | query | No | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'getPagesPublic Przykład'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć własnego klienta HTTP, przekaż swojego klienta implementującego `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie używany będzie `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Niewidoczny wskaźnik stronicowania zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`.
    'limit' => 56, // int | 1..200, domyślnie 50
    'q' => 'q_example', // string | Opcjonalny filtr prefiksu tytułu, nie rozróżniający wielkości liter.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze najpierw), `commentCount` (najwięcej komentarzy najpierw) lub `title` (alfabetycznie).
    'has_comments' => True, // bool | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]