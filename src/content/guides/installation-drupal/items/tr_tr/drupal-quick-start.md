Bu, Drupal talimatlarının "çok uzun; okumadım" sürümüdür.

1. Modülü `composer require drupal/fcom` ile yükleyin veya `modules/custom/fastcomments/` dizinine koyun.
2. `drush en fastcomments` ile etkinleştirin veya yönetici arayüzünden `/admin/modules` üzerinden etkinleştirin.
3. `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`) adresine gidin.
4. Tenant ID ve API Secret bilgilerinizi [Ayarlar > API/SSO](https://fastcomments.com/auth/my-account/api) sayfasından girin ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Herhangi bir içerik türüne `FastComments` alanını `Structure > Content types > [type] > Manage fields` üzerinden ekleyin.

Modül [drupal.org/project/fcom](https://www.drupal.org/project/fcom) adresinde yayınlanmıştır.