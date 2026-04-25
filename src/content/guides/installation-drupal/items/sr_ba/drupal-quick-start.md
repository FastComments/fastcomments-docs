Ovo je TL;DR (skraćena) verzija uputstava za Drupal.

1. Instalirajte modul sa `composer require drupal/fcom`, ili ga stavite u `modules/custom/fastcomments/`.
2. Omogućite ga sa `drush en fastcomments`, ili iz administratorskog UI na `/admin/modules`.
3. Idite na `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Unesite vaš Tenant ID i API Secret iz [Postavke > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Dodajte polje `FastComments` u bilo koji tip sadržaja putem `Structure > Content types > [type] > Manage fields`.

Modul je objavljen na [drupal.org/project/fcom](https://www.drupal.org/project/fcom).