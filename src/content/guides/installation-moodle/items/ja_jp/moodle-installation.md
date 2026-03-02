#### プラグインをダウンロード

最新のリリースZIPを <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub リポジトリ</a> からダウンロードしてください。

#### Moodle ディレクトリに展開する

プラグインが `<moodle-root>/local/fastcomments` に配置されるように、ZIPをMoodleのインストール先に展開してください。プラグインディレクトリには `version.php`、`lib.php`、およびその他のプラグインファイルが直接含まれている必要があります（サブフォルダにネストしないこと）。

例えば：

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Moodle 管理画面からインストール

サイト管理者としてログインし、**サイト管理 > 通知** に移動してください。Moodleは新しいプラグインを検出し、インストールを実行するよう促します。

#### プラグインの設定

インストール後、設定を入力するには **サイト管理 > プラグイン > ローカルプラグイン > FastComments** に移動してください。各オプションの詳細については [設定](#moodle-configuration) セクションを参照してください。