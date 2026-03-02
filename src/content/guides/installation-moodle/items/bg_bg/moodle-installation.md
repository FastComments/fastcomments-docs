#### Изтеглете плъгина

Изтеглете най-новото издание в ZIP формат от <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">репозитория FastComments Moodle в GitHub</a>.

#### Разархивирайте в директорията на Moodle

Разархивирайте ZIP файла в инсталацията на Moodle така, че приставката да се намира в `<moodle-root>/local/fastcomments`. Директорията на приставката трябва да съдържа `version.php`, `lib.php` и други файлове на приставката директно (не вложени в подпапка).

Например:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Инсталиране чрез администраторския панел на Moodle

Влезте като администратор на сайта и отидете на **Администриране на сайта > Известия**. Moodle ще открие новия плъгин и ще ви подкани да стартирате инсталацията.

#### Конфигуриране на плъгина

След инсталацията отидете на **Администриране на сайта > Плъгини > Локални плъгини > FastComments**, за да въведете настройките си. Вижте секцията [Конфигурация](#items-moodle-configuration) за подробности за всяка опция.

---