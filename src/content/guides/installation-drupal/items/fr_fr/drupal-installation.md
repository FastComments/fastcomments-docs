Le module FastComments pour Drupal remplace les commentaires intégrés de Drupal par un système de commentaires rapide et en temps réel. Le module est [publié sur drupal.org](https://www.drupal.org/project/fcom) et fonctionne avec Drupal 10 et 11.

Il existe deux manières de l'installer.

## Installation avec Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Installation manuelle

Téléchargez le module depuis [drupal.org/project/fcom](https://www.drupal.org/project/fcom) et placez-le dans le répertoire `modules/custom/fastcomments/` de votre site. Ensuite, activez-le avec `drush en fastcomments`, ou depuis l'interface d'administration à `Extend` (`/admin/modules`).

<sup>Remarque !</sup> Le module ne dépend que du noyau de Drupal (`user` et `field`). Aucun autre module Drupal ou bibliothèque n'est requis.

Une fois le module activé, rendez-vous dans la section `Configuration` pour configurer votre Tenant ID et API Secret.

---