Lister les pages pour un locataire. Utilisé par le client de bureau FChat pour peupler sa liste de salons.
Requiert que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.
Les pages qui nécessitent SSO sont filtrées en fonction de l'accès de groupe de l'utilisateur demandeur.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Curseur de pagination opaque renvoyé comme `nextCursor` par une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | No | 1..200, par défaut 50 |
| q | string | query | No | Filtre optionnel par préfixe de titre, insensible à la casse. |
| sortBy | string | query | No | Ordre de tri. `updatedAt` (par défaut, le plus récent en premier), `commentCount` (le plus de commentaires en premier), ou `title` (alphabétique). |
| hasComments | boolean | query | No | Si vrai, ne retourner que les pages ayant au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Curseur de pagination opaque renvoyé comme `nextCursor` par une requête précédente. Lié au même `sortBy`.
$limit = 56; // int | 1..200, par défaut 50
$q = 'q_example'; // string | Filtre optionnel par préfixe de titre, insensible à la casse.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Ordre de tri. `updatedAt` (par défaut, le plus récent en premier), `commentCount` (le plus de commentaires en premier), ou `title` (alphabétique).
$has_comments = True; // bool | Si vrai, ne retourner que les pages ayant au moins un commentaire.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]