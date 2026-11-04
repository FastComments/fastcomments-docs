Liste les pages d'un tenant. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Nécessite que `enableFChat` soit à true dans la configuration personnalisée résolue pour chaque page.
Les pages qui requièrent SSO sont filtrées selon les accès de groupe de l'utilisateur effectuant la requête.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| cursor | string | query | Non | Curseur de pagination opaque renvoyé sous forme de `nextCursor` par une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | Non | 1..200, valeur par défaut 50 |
| q | string | query | Non | Filtre facultatif de préfixe de titre, insensible à la casse. |
| sortBy | string | query | Non | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (le plus grand nombre de commentaires d'abord), ou `title` (alphabétique). |
| hasComments | boolean | query | Non | Si true, renvoie uniquement les pages ayant au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client http personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est facultatif, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Curseur de pagination opaque renvoyé sous forme de `nextCursor` par une requête précédente. Lié au même `sortBy`.
    'limit' => 56, // int | 1..200, valeur par défaut 50
    'q' => 'q_example', // string | Filtre facultatif de préfixe de titre, insensible à la casse.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (le plus grand nombre de commentaires d'abord), ou `title` (alphabétique).
    'has_comments' => True, // bool | Si true, renvoie uniquement les pages ayant au moins un commentaire.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]