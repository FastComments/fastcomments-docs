---
El módulo FastComments para Drupal reemplaza los comentarios integrados de Drupal por un sistema de comentarios en tiempo real y rápido. El módulo está [publicado en drupal.org](https://www.drupal.org/project/fcom) y es compatible con Drupal 10 y 11.

Hay dos formas de instalarlo.

## Install with Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Install manually

Descargue el módulo desde [drupal.org/project/fcom](https://www.drupal.org/project/fcom) y colóquelo en el directorio `modules/custom/fastcomments/` de su sitio. Luego actívelo con `drush en fastcomments`, o desde la interfaz de administración en `Extend` (`/admin/modules`).

<sup>¡Nota!</sup> El módulo solo depende del núcleo de Drupal (`user` y `field`). No se requieren otros módulos o bibliotecas de Drupal.

Una vez habilitado el módulo, diríjase a la sección `Configuration` para configurar su Tenant ID y API Secret.

---