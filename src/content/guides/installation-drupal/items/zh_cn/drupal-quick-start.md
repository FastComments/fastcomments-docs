这是 Drupal 指南的 "长话短说" 版本。

1. 使用 `composer require drupal/fcom` 安装该模块，或将其放入 `modules/custom/fastcomments/`。
2. 使用 `drush en fastcomments` 启用，或在管理员界面 `/admin/modules` 中启用。
3. 前往 `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`)。
4. 从 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）获取你的 Tenant ID 和 API Secret，并输入它们。
5. 通过 `Structure > Content types > [type] > Manage fields` 将 `FastComments` 字段添加到任意内容类型。

该模块发布于 [drupal.org/project/fcom](https://www.drupal.org/project/fcom).