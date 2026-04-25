FastComments Drupal-modulet erstatter Drupals indbyggede kommentarer med et hurtigt kommentarsystem i realtid. Modulet er [udgivet på drupal.org](https://www.drupal.org/project/fcom) og fungerer med Drupal 10 og 11.

Der er to måder at installere det på.

## Installér med Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Installer manuelt

Hent modulet fra [drupal.org/project/fcom](https://www.drupal.org/project/fcom) og placer det i din sites `modules/custom/fastcomments/`-mappe. Aktivér det derefter med `drush en fastcomments`, eller fra admin-UI'en under `Extend` (`/admin/modules`).

<sup>Bemærk!</sup> Modulet afhænger kun af Drupal-kernen (`user` og `field`). Der er ingen andre Drupal-moduler eller biblioteker påkrævet.

Når modulet er aktiveret, gå til `Configuration`-sektionen for at konfigurere din Tenant ID og API Secret.