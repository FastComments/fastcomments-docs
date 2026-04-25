De FastComments Drupal-module vervangt de ingebouwde reacties van Drupal door een snel, real-time reactiesysteem. De module is [gepubliceerd op drupal.org](https://www.drupal.org/project/fcom) en werkt met Drupal 10 en 11.

Er zijn twee manieren om het te installeren.

## Installeren met Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Handmatig installeren

Download de module van [drupal.org/project/fcom](https://www.drupal.org/project/fcom) en plaats deze in de map `modules/custom/fastcomments/` van uw site. Schakel het vervolgens in met `drush en fastcomments`, of via de beheer-UI bij `Extend` (`/admin/modules`).

<sup>Opmerking!</sup> De module is alleen afhankelijk van Drupal core (`user` en `field`). Er zijn geen andere Drupal-modules of bibliotheken vereist.

Zodra de module is ingeschakeld, gaat u naar de sectie `Configuration` om uw Tenant ID en API Secret in te stellen.