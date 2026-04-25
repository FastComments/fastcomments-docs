The FastComments Drupal modul zamjenjuje ugrađene Drupal komentare brzim sistemom komentiranja u realnom vremenu. Modul je [objavljen na drupal.org](https://www.drupal.org/project/fcom) i radi sa Drupal 10 i 11.

Postoje dva načina za instalaciju.

## Instalacija pomoću Composer-a

```
composer require drupal/fcom
drush en fastcomments
```

## Ručna instalacija

Preuzmite modul sa [drupal.org/project/fcom](https://www.drupal.org/project/fcom) i postavite ga u direktorij vaše stranice `modules/custom/fastcomments/`. Zatim ga omogućite pomoću `drush en fastcomments`, ili iz administratorskog sučelja na `Extend` (`/admin/modules`).

<sup>Napomena!</sup> Modul zavisi samo od Drupal core-a (`user` i `field`). Nisu potrebni drugi Drupal moduli ili biblioteke.

Kada je modul omogućen, idite u sekciju `Configuration` da podesite vaš Tenant ID i API Secret.