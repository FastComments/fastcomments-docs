Das FastComments-Drupal-Modul ersetzt die eingebauten Kommentare von Drupal durch ein schnelles, Echtzeit-Kommentarsystem. Das Modul ist [auf drupal.org veröffentlicht](https://www.drupal.org/project/fcom) und funktioniert mit Drupal 10 und 11.

Es gibt zwei Möglichkeiten, es zu installieren.

## Installation mit Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Manuelle Installation

Laden Sie das Modul von [drupal.org/project/fcom](https://www.drupal.org/project/fcom) herunter und legen Sie es im Verzeichnis Ihrer Website `modules/custom/fastcomments/` ab. Aktivieren Sie es dann mit `drush en fastcomments`, oder über die Administrationsoberfläche unter `Extend` (`/admin/modules`).

<sup>Hinweis!</sup> Das Modul hängt nur vom Drupal-Core (`user` und `field`) ab. Es sind keine weiteren Drupal-Module oder Bibliotheken erforderlich.

Sobald das Modul aktiviert ist, gehen Sie zum Abschnitt `Configuration`, um Ihre Tenant ID und Ihr API Secret einzurichten.

---