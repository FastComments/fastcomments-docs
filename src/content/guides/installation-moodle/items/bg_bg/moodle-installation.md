#### Изтеглете приставката

Изтеглете най-новия ZIP архив на изданието от <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub хранилището</a>.

#### Разархивирайте в директорията на вашия Moodle

Разархивирайте ZIP архива в инсталацията на вашия Moodle така че приставката да се намира в `<moodle-root>/local/fastcomments`. Директорията на приставката трябва да съдържа `version.php`, `lib.php`, и други файлове на приставката директно (не вложени в подпапка).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Инсталиране чрез администратора на Moodle

Влезте като администратор на сайта и отидете в **Администриране на сайта > Известия**. Moodle ще открие новата приставка и ще ви подкани да стартирате инсталацията.

#### Конфигуриране на приставката

След инсталацията отидете в **Администриране на сайта > Плъгини > Локални плъгини > FastComments**, за да въведете вашите настройки. Вижте секцията [Конфигурация](#moodle-configuration) за подробности за всяка опция.