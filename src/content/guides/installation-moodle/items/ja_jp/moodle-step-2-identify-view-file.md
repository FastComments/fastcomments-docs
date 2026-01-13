次に、`view.php` を特定する必要があります。これはおそらく `/var/www/html/moodle/mod/book/view.php` のようなディレクトリにあります。

このファイルを見つけたら、先に進む前にバックアップを取りましょう: `sudo cp /var/www/html/moodle/mod/book/view.php /var/www/html/moodle/mod/book/view.php.bak`.

このチュートリアルの途中でサイトが壊れた場合は、次のコマンドで復元できます: `sudo cp /var/www/html/moodle/mod/book/view.php.bak /var/www/html/moodle/mod/book/view.php`.