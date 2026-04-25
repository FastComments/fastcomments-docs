這是 Drupal 說明的「太長不看」版本。

1. 使用 `composer require drupal/fcom` 安裝模組，或將其放入 `modules/custom/fastcomments/`。
2. 使用 `drush en fastcomments` 啟用，或在管理介面 `/admin/modules` 中啟用。
3. 前往 `Administration > Configuration > Content > FastComments`（`/admin/config/content/fastcomments`）。
4. 在 [Settings > API/SSO](https://fastcomments.com/auth/my-account/api)（[EU](https://eu.fastcomments.com/auth/my-account/api)）中輸入您的 Tenant ID 與 API Secret。
5. 透過 `Structure > Content types > [type] > Manage fields` 將 `FastComments` 欄位新增到任一內容類型。

該模組已發佈於 [drupal.org/project/fcom](https://www.drupal.org/project/fcom)。