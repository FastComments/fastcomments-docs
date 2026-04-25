Il modulo FastComments per Drupal sostituisce i commenti integrati di Drupal con un sistema di commenti in tempo reale e veloce. Il modulo è [pubblicato su drupal.org](https://www.drupal.org/project/fcom) e funziona con Drupal 10 e 11.

Ci sono due modi per installarlo.

## Installazione con Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Installazione manuale

Scarica il modulo da [drupal.org/project/fcom](https://www.drupal.org/project/fcom) e posizionalo nella directory `modules/custom/fastcomments/` del tuo sito. Quindi abilitalo con `drush en fastcomments`, oppure dall'interfaccia di amministrazione in `Extend` (`/admin/modules`).

<sup>Nota!</sup> Il modulo dipende solo dal core di Drupal (`user` and `field`). Non sono richiesti altri moduli Drupal o librerie.

Una volta abilitato il modulo, vai alla sezione `Configuration` per impostare il Tenant ID e l'API Secret.