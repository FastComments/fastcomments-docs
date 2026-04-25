---
FastComments Drupal 模块将 Drupal 内置的评论替换为一个快速、实时的评论系统。该模块已在 [drupal.org](https://www.drupal.org/project/fcom) 发布，并适用于 Drupal 10 和 11。

有两种安装方式。

## 使用 Composer 安装

```
composer require drupal/fcom
drush en fastcomments
```

## 手动安装

从 [drupal.org/project/fcom](https://www.drupal.org/project/fcom) 下载该模块，并将其放到站点的 `modules/custom/fastcomments/` 目录下。然后使用 `drush en fastcomments` 启用，或从管理界面的 `Extend`（`/admin/modules`）中启用。

<sup>注意！</sup> 该模块仅依赖于 Drupal 核心（`user` 和 `field`）。不需要其他 Drupal 模块或库。

启用模块后，前往 `Configuration` 部分以设置您的 Tenant ID 和 API Secret。

---