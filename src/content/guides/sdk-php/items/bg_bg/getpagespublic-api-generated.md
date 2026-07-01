List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачен курсор за пагинация, връщан като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Опционален филтър за заглавие, нечувствителен към регистъра. |
| sortBy | string | query | No | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първо), `commentCount` (най-много коментари първо), или `title` (по азбучен ред). |
| hasComments | boolean | query | No | Ако е true, връща само страници с поне един коментар. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'getPagesPublic пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`.
    'limit' => 56, // int | 1..200, default 50
    'q' => 'q_example', // string | Optional case-insensitive title prefix filter.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical).
    'has_comments' => True, // bool | If true, only return pages with at least one comment.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]