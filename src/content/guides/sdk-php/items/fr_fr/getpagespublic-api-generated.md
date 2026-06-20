Lister les pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salles.
Nécessite que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.
Les pages nécessitant SSO sont filtrées en fonction de l'accès aux groupes de l'utilisateur demandeur.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| cursor | string | requête | Non | Curseur de pagination opaque renvoyé comme `nextCursor` par une requête précédente. Lié au même `sortBy`. |
| limit | integer | requête | Non | 1..200, par défaut 50 |
| q | string | requête | Non | Filtre optionnel sur le préfixe du titre, insensible à la casse. |
| sortBy | string | requête | Non | Ordre de tri. `updatedAt` (par défaut, le plus récent en premier), `commentCount` (le plus de commentaires en premier), ou `title` (alphabétique). |
| hasComments | boolean | requête | Non | Si vrai, ne renvoyer que les pages ayant au moins un commentaire. |

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
$q = 'q_example'; // string | Filtre optionnel sur le préfixe du titre, insensible à la casse.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Ordre de tri. `updatedAt` (par défaut, le plus récent en premier), `commentCount` (le plus de commentaires en premier), ou `title` (alphabétique).
$has_comments = True; // bool | Si vrai, ne renvoyer que les pages ayant au moins un commentaire.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]