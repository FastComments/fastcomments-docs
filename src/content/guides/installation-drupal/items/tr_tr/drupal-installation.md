FastComments Drupal modülü, Drupal'ın yerleşik yorum sistemini hızlı, gerçek zamanlı bir yorum sistemiyle değiştirir. Modül [drupal.org'da yayımlandı](https://www.drupal.org/project/fcom) ve Drupal 10 ile 11 ile çalışır.

Kurmanın iki yolu vardır.

## Composer ile Kurulum

```
composer require drupal/fcom
drush en fastcomments
```

## Manuel Kurulum

Modülü [drupal.org/project/fcom](https://www.drupal.org/project/fcom) adresinden indirin ve sitenizin `modules/custom/fastcomments/` dizinine yerleştirin. Ardından `drush en fastcomments` ile etkinleştirin veya yönetici arayüzünden `Extend` (`/admin/modules`) üzerinden etkinleştirin.

<sup>Not:</sup> Modül yalnızca Drupal çekirdeğine (`user` ve `field`) bağımlıdır. Başka Drupal modülleri veya kütüphaneler gerekli değildir.

Modül etkinleştirildikten sonra, Tenant ID ve API Secret'inizi ayarlamak için `Configuration` bölümüne gidin.