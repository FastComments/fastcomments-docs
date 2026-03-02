#### 下载插件

从 <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub 仓库</a> 下载最新发布的 ZIP。

#### 解压到您的 Moodle 目录

将 ZIP 解压到您的 Moodle 安装目录，使插件位于 `<moodle-root>/local/fastcomments`。插件目录应直接包含 `version.php`、`lib.php` 和其他插件文件（不要嵌套在子文件夹中）。

例如：

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### 通过 Moodle 管理界面安装

以站点管理员身份登录，然后导航到 **站点管理 > 通知**。Moodle 会检测到新插件并提示您运行安装。

#### 配置插件

安装完成后，转到 **站点管理 > 插件 > 本地插件 > FastComments** 以输入您的设置。有关每个选项的详细信息，请参阅 [配置](#items-moodle-configuration) 部分。

---