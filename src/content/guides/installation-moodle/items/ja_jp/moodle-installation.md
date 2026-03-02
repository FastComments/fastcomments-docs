#### プラグインをダウンロード

最新リリースの ZIP を <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub リポジトリ</a> からダウンロードしてください。

#### Moodle ディレクトリに展開する

ZIP をあなたの Moodle インストールに展開し、プラグインが `<moodle-root>/local/fastcomments` に配置されるようにします。プラグインディレクトリには `version.php`、`lib.php`、およびその他のプラグインファイルが直接（サブフォルダにネストされていない状態で）含まれている必要があります。

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Moodle 管理画面からインストール

サイト管理者としてログインし、**サイト管理 > 通知** に移動してください。Moodle が新しいプラグインを検出し、インストールの実行を促します。

#### プラグインの設定

インストール後、設定を入力するには **サイト管理 > プラグイン > ローカルプラグイン > FastComments** に移動してください。各オプションの詳細は [設定](#moodle-configuration) セクションを参照してください。