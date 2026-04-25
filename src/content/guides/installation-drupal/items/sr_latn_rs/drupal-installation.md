FastComments Drupal modul zamenjuje ugrađene komentare Drupala brzim sistemom za komentarisanje u realnom vremenu. Modul je [objavljen na drupal.org](https://www.drupal.org/project/fcom) i radi sa Drupalom 10 i 11.

Postoje dva načina za instalaciju.

## Instalacija pomoću Composera

```
composer require drupal/fcom
drush en fastcomments
```

## Ručna instalacija

Preuzmite modul sa [drupal.org/project/fcom](https://www.drupal.org/project/fcom) i postavite ga u direktorijum vašeg sajta `modules/custom/fastcomments/`. Zatim ga omogućite pomoću `drush en fastcomments`, ili iz administratorskog interfejsa pod `Extend` (`/admin/modules`).

<sup>Napomena!</sup> Modul zavisi samo od Drupal core-a (`user` i `field`). Nisu potrebni drugi Drupal moduli ili biblioteke.

Kada je modul omogućen, idite na odeljak `Configuration` da podesite vaš Tenant ID i API Secret.