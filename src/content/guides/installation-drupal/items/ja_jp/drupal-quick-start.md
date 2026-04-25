---
これは Drupal の説明の「要点のみ（TL;DR）」版です。

1. モジュールを `composer require drupal/fcom` でインストールするか、`modules/custom/fastcomments/` に配置します。
2. `drush en fastcomments` で有効化するか、管理 UI の `/admin/modules` から有効化します。
3. `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`) に移動します。
4. [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）から Tenant ID と API Secret を入力します。
5. `Structure > Content types > [type] > Manage fields` 経由で任意のコンテンツタイプに `FastComments` フィールドを追加します。

モジュールは [drupal.org/project/fcom](https://www.drupal.org/project/fcom) に公開されています。

---