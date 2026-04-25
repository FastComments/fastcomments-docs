O módulo FastComments para Drupal substitui os comentários nativos do Drupal por um sistema de comentários em tempo real, rápido. O módulo está [publicado no drupal.org](https://www.drupal.org/project/fcom) e funciona com Drupal 10 e 11.

Existem duas maneiras de instalá-lo.

## Instalar com Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Instalar manualmente

Faça o download do módulo em [drupal.org/project/fcom](https://www.drupal.org/project/fcom) e coloque-o no diretório `modules/custom/fastcomments/` do seu site. Em seguida, habilite-o com `drush en fastcomments`, ou pela interface de administração em `Extend` (`/admin/modules`).

<sup>Observação!</sup> O módulo depende apenas do core do Drupal (`user` e `field`). Não são necessários outros módulos do Drupal ou bibliotecas.

Uma vez que o módulo for habilitado, vá para a seção `Configuration` para configurar seu Tenant ID e API Secret.