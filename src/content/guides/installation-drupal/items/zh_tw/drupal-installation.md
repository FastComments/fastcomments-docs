FastComments Drupal 模組以快速、即時的留言系統取代 Drupal 內建的留言功能。該模組已[發佈於 drupal.org](https://www.drupal.org/project/fcom),並支援 Drupal 10 與 11。

有兩種安裝方式。

## 使用 Composer 安裝

```
composer require drupal/fcom
drush en fastcomments
```

## 手動安裝

從 [drupal.org/project/fcom](https://www.drupal.org/project/fcom) 下載模組,並將其放置於您網站的 `modules/custom/fastcomments/` 目錄中。然後使用 `drush en fastcomments` 啟用它,或從管理介面的 `Extend`(`/admin/modules`)中啟用。

<sup>注意!</sup> 此模組僅依賴 Drupal 核心(`user` 與 `field`)。不需要其他 Drupal 模組或函式庫。

模組啟用後,請前往 `Configuration` 區段設定您的 Tenant ID 與 API Secret。