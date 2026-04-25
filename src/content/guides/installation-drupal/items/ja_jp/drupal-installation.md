The FastComments Drupal モジュールは、Drupal の組み込みコメントを高速でリアルタイムなコメントシステムに置き換えます。モジュールは [drupal.org に公開されています](https://www.drupal.org/project/fcom) と Drupal 10 および 11 に対応しています。

There are two ways to install it.

## Install with Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Install manually

Download the module from [drupal.org/project/fcom](https://www.drupal.org/project/fcom) and place it in your site's `modules/custom/fastcomments/` directory. Then enable it with `drush en fastcomments`, or from the admin UI at `Extend` (`/admin/modules`).

<sup>注：</sup> このモジュールは Drupal コア（`user` および `field`）のみを依存関係としています。他の Drupal モジュールやライブラリは必要ありません。

Once the module is enabled, head to the `Configuration` section to set up your Tenant ID and API Secret.