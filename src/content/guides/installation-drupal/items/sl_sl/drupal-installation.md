Modul FastComments za Drupal nadomešča vgrajene komentarje Drupala s hitrim, v realnem času delujočim sistemom za komentiranje. Modul je [objavljen na drupal.org](https://www.drupal.org/project/fcom) in deluje z Drupalom 10 in 11.

Obstajata dva načina za namestitev.

## Namestitev z Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Ročna namestitev

Prenesite modul z [drupal.org/project/fcom](https://www.drupal.org/project/fcom) in ga postavite v mapo vaše spletne strani `modules/custom/fastcomments/`. Nato ga omogočite z `drush en fastcomments`, ali iz skrbniškega vmesnika na `Extend` (`/admin/modules`).

<sup>Note!</sup> Modul je odvisen le od Drupal core (`user` and `field`). Ni potrebnih nobenih drugih Drupal modulov ali knjižnic.

Ko je modul omogočen, pojdite v razdelek `Configuration`, da nastavite svoj Tenant ID in API Secret.