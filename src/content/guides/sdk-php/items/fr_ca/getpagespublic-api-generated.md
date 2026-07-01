List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Curseur de pagination opaque renvoyé comme `nextCursor` à partir d'une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | No | 1..200, défaut 50 |
| q | string | query | No | Filtre de préfixe de titre optionnel, insensible à la casse. |
| sortBy | string | query | No | Ordre de tri. `updatedAt` (défaut, le plus récent d'abord), `commentCount` (le plus de commentaires d'abord), ou `title` (alphabétique). |
| hasComments | boolean | query | No | Si vrai, ne renvoie que les pages contenant au moins un commentaire. |

## Response

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous voulez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // chaîne
$options = [
    'cursor' => 'cursor_example', // chaîne | Curseur de pagination opaque renvoyé comme `nextCursor` à partir d'une requête précédente. Lié au même `sortBy`.
    'limit' => 56, // int | 1..200, défaut 50
    'q' => 'q_example', // chaîne | Filtre de préfixe de titre optionnel, insensible à la casse.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Ordre de tri. `updatedAt` (défaut, le plus récent d'abord), `commentCount` (le plus de commentaires d'abord), ou `title` (alphabétique).
    'has_comments' => True, // bool | Si vrai, ne renvoie que les pages contenant au moins un commentaire.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]