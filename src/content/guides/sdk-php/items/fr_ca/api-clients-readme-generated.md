Le SDK expose trois classes clientes API :

- **`DefaultApi`** — méthodes authentifiées par clé API pour une utilisation côté serveur. Configurez une clé API comme indiqué dans [Démarrage](#getting-started-readme-generated).
- **`PublicApi`** — méthodes publiques qui ne nécessitent pas de clé API, sûres à appeler depuis des navigateurs et des applications mobiles.
- **`ModerationApi`** — méthodes pour le tableau de bord du modérateur : lister, compter, rechercher, consigner et exporter les commentaires ; actions de modération (supprimer/restaurer, signaler, définir l’état revue/spam/approbation, votes, rouvrir/fermer un fil) ; bannissements (interdiction de commenter, annulation, résumés pré-bannissement, état et préférences de bannissement, nombre d’utilisateurs bannis) ; et badges & confiance (attribuer/supprimer un badge, badges manuels, obtenir/définir le facteur de confiance, profil interne de l’utilisateur). Chaque méthode de `ModerationApi` accepte un paramètre `$sso` pour authentifier le modérateur agissant via SSO.

### Utilisation de PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Les méthodes publiques ne nécessitent pas de clé API.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // chaîne
$url_id = 'url_id_example'; // chaîne

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Utilisation de ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // chaîne - charge utile SSO authentifiant le modérateur

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```