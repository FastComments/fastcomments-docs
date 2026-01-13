接下来我们需要找到 `view.php`。它很可能位于类似于 `/var/www/html/moodle/mod/book/view.php` 的目录中。

一旦找到该文件，我们在继续之前先备份它：`sudo cp /var/www/html/moodle/mod/book/view.php /var/www/html/moodle/mod/book/view.php.bak`。

如果在本教程过程中站点出现故障，我们可以通过以下命令恢复：`sudo cp /var/www/html/moodle/mod/book/view.php.bak /var/www/html/moodle/mod/book/view.php`。