#### Загрузите плагин

Загрузите последний ZIP-релиз из <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">репозитория FastComments Moodle на GitHub</a>.

#### Распакуйте в каталог Moodle

Распакуйте ZIP в установку Moodle так, чтобы плагин находился по пути `<moodle-root>/local/fastcomments`. Директория плагина должна содержать `version.php`, `lib.php` и другие файлы плагина непосредственно (не вложенные в подпапку).

Например:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Установка через админку Moodle

Войдите под учётной записью администратора сайта и перейдите в **Администрирование сайта > Уведомления**. Moodle обнаружит новый плагин и предложит запустить установку.

#### Настройка плагина

После установки перейдите в **Администрирование сайта > Плагины > Локальные плагины > FastComments**, чтобы ввести ваши настройки. Смотрите раздел [Конфигурация](#moodle-configuration) для подробностей по каждой опции.