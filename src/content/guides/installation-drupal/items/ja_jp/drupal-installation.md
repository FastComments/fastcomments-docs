FastComments Drupalモジュールは、Drupalの組み込みコメントを高速なリアルタイムコメントシステムに置き換えます。このモジュールは[drupal.orgで公開されており](https://www.drupal.org/project/fcom)、Drupal 10および11で動作します。

インストール方法は2つあります。

## Composerでインストール

```
composer require drupal/fcom
drush en fastcomments
```

## 手動でインストール

[drupal.org/project/fcom](https://www.drupal.org/project/fcom)からモジュールをダウンロードし、サイトの`modules/custom/fastcomments/`ディレクトリに配置します。その後、`drush en fastcomments`で有効化するか、管理UIの`Extend`(`/admin/modules`)から有効化してください。

<sup>注意！</sup> このモジュールはDrupalコア(`user`と`field`)にのみ依存しています。他のDrupalモジュールやライブラリは必要ありません。

モジュールを有効化したら、`Configuration`セクションに移動してTenant IDとAPI Secretを設定してください。