FastComments Drupal modul zamjenjuje ugrađene Drupal komentare brzim, komentarskim sustavom u stvarnom vremenu. Modul je [objavljen na drupal.org](https://www.drupal.org/project/fcom) i radi s Drupalom 10 i 11.

Postoje dva načina instalacije.

## Instalacija s Composerom

```
composer require drupal/fcom
drush en fastcomments
```

## Ručna instalacija

Preuzmite modul s [drupal.org/project/fcom](https://www.drupal.org/project/fcom) i smjestite ga u direktorij vaše stranice `modules/custom/fastcomments/`. Zatim ga omogućite s `drush en fastcomments`, ili putem administratorskog sučelja pod `Extend` (`/admin/modules`).

<sup>Napomena!</sup> Modul ovisi samo o jezgri Drupala (`user` i `field`). Nisu potrebni drugi Drupal moduli niti biblioteke.

Kada je modul omogućen, idite u odjeljak `Configuration` kako biste postavili svoj Tenant ID i API Secret.