---
FastComments 的 Drupal 模組用快速、即時的留言系統取代 Drupal 內建的留言功能。該模組已在 [已在 drupal.org 發佈](https://www.drupal.org/project/fcom)，支援 Drupal 10 與 11。

有兩種安裝方式。

## Install with Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Install manually

從 [drupal.org/project/fcom](https://www.drupal.org/project/fcom) 下載模組，並將它放到你網站的 `modules/custom/fastcomments/` 目錄。然後使用 `drush en fastcomments` 啟用，或從管理介面的 `Extend` (`/admin/modules`) 啟用。

<sup>注意！</sup> 模組只依賴 Drupal 核心（`user` 和 `field`）。不需要其他 Drupal 模組或函式庫。

啟用模組後，前往 `Configuration` 區段以設定你的 Tenant ID 和 API Secret。

---