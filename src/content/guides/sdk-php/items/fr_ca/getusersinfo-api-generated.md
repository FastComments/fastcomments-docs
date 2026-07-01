Informations utilisateur en masse pour un locataire. Étant donné des userIds, renvoie les informations d’affichage provenant de User / SSOUser.  
Utilisé par le widget de commentaires pour enrichir les utilisateurs qui viennent d’apparaître via un événement de présence.  
Pas de contexte de page : la confidentialité est appliquée uniformément (les profils privés sont masqués).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Identifiants d’utilisateurs séparés par des virgules. |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | Identifiants d’utilisateurs séparés par des virgules.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---