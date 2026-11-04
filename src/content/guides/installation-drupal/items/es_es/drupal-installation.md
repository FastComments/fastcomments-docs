El módulo de FastComments para Drupal reemplaza los comentarios integrados de Drupal con un sistema de comentarios rápido y en tiempo real. El módulo está [publicado en drupal.org](https://www.drupal.org/project/fcom) y funciona con Drupal 10 y 11.

Hay dos formas de instalarlo.

## Instalar con Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Instalación manual

Descarga el módulo desde [drupal.org/project/fcom](https://www.drupal.org/project/fcom) y colócalo en el directorio `modules/custom/fastcomments/` de tu sitio. Luego actívalo con `drush en fastcomments`, o desde la interfaz de administración en `Extend` (`/admin/modules`).

<sup>¡Nota!</sup> El módulo solo depende del núcleo de Drupal (`user` y `field`). No se requieren otros módulos ni librerías de Drupal.

Una vez que el módulo esté activado, ve a la sección `Configuration` para configurar tu Tenant ID y API Secret.