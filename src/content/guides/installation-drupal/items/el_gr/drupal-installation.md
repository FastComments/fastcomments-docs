Το module FastComments για το Drupal αντικαθιστά τα ενσωματωμένα σχόλια του Drupal με ένα γρήγορο, σε πραγματικό χρόνο σύστημα σχολιασμού. Το module είναι [δημοσιευμένο στο drupal.org](https://www.drupal.org/project/fcom) και λειτουργεί με Drupal 10 και 11.

Υπάρχουν δύο τρόποι εγκατάστασης.

## Εγκατάσταση μέσω Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Εγκατάσταση χειροκίνητα

Κατεβάστε το module από [drupal.org/project/fcom](https://www.drupal.org/project/fcom) και τοποθετήστε το στον κατάλογο `modules/custom/fastcomments/` του ιστότοπού σας. Στη συνέχεια ενεργοποιήστε το με `drush en fastcomments`, ή από το γραφικό περιβάλλον διαχείρισης στο `Extend` (`/admin/modules`).

<sup>Σημείωση!</sup> Το module εξαρτάται μόνο από τον πυρήνα του Drupal (`user` και `field`). Δεν απαιτούνται άλλα modules ή βιβλιοθήκες του Drupal.

Μόλις ενεργοποιηθεί το module, μεταβείτε στην ενότητα `Configuration` για να ρυθμίσετε το Tenant ID και το API Secret.