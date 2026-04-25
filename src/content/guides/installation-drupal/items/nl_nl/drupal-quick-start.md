---
Dit is de "te lang; niet gelezen" versie van de Drupal-instructies.

1. Installeer de module met `composer require drupal/fcom`, of plaats deze in `modules/custom/fastcomments/`.
2. Schakel deze in met `drush en fastcomments`, of via de admin-UI op `/admin/modules`.
3. Ga naar `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Voer uw Tenant ID en API Secret in van [Instellingen > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Voeg het veld `FastComments` toe aan elk contenttype via `Structure > Content types > [type] > Manage fields`.

De module is gepubliceerd op [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---