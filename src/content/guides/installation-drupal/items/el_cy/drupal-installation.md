Το FastComments module για Drupal αντικαθιστά τα ενσωματωμένα σχόλια του Drupal με ένα γρήγορο, σε πραγματικό χρόνο σύστημα σχολιασμού. Το module είναι [δημοσιευμένο στο drupal.org](https://www.drupal.org/project/fcom) και λειτουργεί με Drupal 10 και 11.

There are two ways to install it.

## Εγκατάσταση με Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Χειροκίνητη εγκατάσταση

Κατεβάστε το module από [drupal.org/project/fcom](https://www.drupal.org/project/fcom) και τοποθετήστε το στον φάκελο `modules/custom/fastcomments/` του ιστότοπού σας. Στη συνέχεια ενεργοποιήστε το με `drush en fastcomments`, ή από το διαχειριστικό περιβάλλον στο `Extend` (`/admin/modules`).

<sup>Σημείωση!</sup> Το module εξαρτάται μόνο από τον πυρήνα του Drupal (`user` και `field`). Δεν απαιτούνται άλλα Drupal modules ή βιβλιοθήκες.

Μόλις το module ενεργοποιηθεί, μεταβείτε στην ενότητα `Configuration` για να ρυθμίσετε το Tenant ID και το API Secret.