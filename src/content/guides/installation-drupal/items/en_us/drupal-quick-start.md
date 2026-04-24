This is the "too long did not read" version of the Drupal instructions.

1. Install the module with `composer require drupal/fcom`, or drop it in `modules/custom/fastcomments/`.
2. Enable it with `drush en fastcomments`, or from the admin UI at `/admin/modules`.
3. Go to `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Enter your Tenant ID and API Secret from [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Add the `FastComments` field to any content type via `Structure > Content types > [type] > Manage fields`.

The module is published at [drupal.org/project/fcom](https://www.drupal.org/project/fcom).
