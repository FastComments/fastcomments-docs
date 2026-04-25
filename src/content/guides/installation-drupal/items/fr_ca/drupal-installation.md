Le module FastComments pour Drupal remplace le système de commentaires intégré de Drupal par un système de commentaires rapide et en temps réel. Le module est [publié sur drupal.org](https://www.drupal.org/project/fcom) et fonctionne avec Drupal 10 et 11.

Il existe deux façons de l'installer.

## Installer avec Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Installer manuellement

Téléchargez le module depuis [drupal.org/project/fcom] et placez-le dans le répertoire `modules/custom/fastcomments/` de votre site. Ensuite, activez-le avec `drush en fastcomments`, ou via l'interface d'administration à `Extend` (`/admin/modules`).

<sup>Remarque!</sup> Le module ne dépend que du noyau de Drupal (`user` et `field`). Aucun autre module ou bibliothèque Drupal n'est requis.

Une fois le module activé, allez à la section `Configuration` pour configurer votre Tenant ID et API Secret.