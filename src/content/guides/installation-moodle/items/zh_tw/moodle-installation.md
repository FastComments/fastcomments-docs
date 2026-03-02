#### 下載外掛

從 <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub 儲存庫</a> 下載最新發行的 ZIP。

#### 解壓到你的 Moodle 目錄

將 ZIP 解壓到你的 Moodle 安裝目錄，讓外掛位於 `<moodle-root>/local/fastcomments`。外掛目錄應直接包含 `version.php`、`lib.php`，和其他外掛檔案（不可巢狀於子資料夾）。

例如：

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### 透過 Moodle 管理介面安裝

以網站管理員身分登入，然後前往 **網站管理 > 通知**。Moodle 會偵測到新的外掛並提示您執行安裝。

#### 設定外掛

安裝後，前往 **網站管理 > 外掛 > 本地外掛 > FastComments** 以輸入您的設定。請參閱 [設定](#moodle-configuration) 區段以了解各選項的詳細說明。