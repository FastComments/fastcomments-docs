---
To je skrajšana različica (TL;DR) navodil za Drupal.

1. Namestite modul z `composer require drupal/fcom`, ali ga postavite v `modules/custom/fastcomments/`.
2. Omogočite ga z `drush en fastcomments`, ali preko skrbniškega vmesnika na `/admin/modules`.
3. Pojdite na `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Vnesite svoj Tenant ID in API Secret iz [Nastavitve > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Dodajte polje `FastComments` kateremu koli tipu vsebine preko `Structure > Content types > [type] > Manage fields`.

Modul je objavljen na [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---