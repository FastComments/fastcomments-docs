#### 下載外掛

從 <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub 儲存庫</a> 下載最新的發行 ZIP 檔。

#### 解壓到您的 Moodle 目錄

將 ZIP 解壓到您的 Moodle 安裝目錄，使外掛位於 `<moodle-root>/local/fastcomments`。外掛目錄應直接包含 `version.php`、`lib.php` 與其他外掛檔案（不應放在子資料夾內）。

例如：

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### 透過 Moodle 管理員介面安裝

以站點管理員身份登入，然後前往 **Site Administration > Notifications**。Moodle 會偵測到新的外掛並提示您執行安裝。

#### 設定外掛

安裝完成後，前往 **Site Administration > Plugins > Local plugins > FastComments** 輸入您的設定。請參閱 [Configuration](#moodle-configuration) 節，了解各個選項的詳細說明。