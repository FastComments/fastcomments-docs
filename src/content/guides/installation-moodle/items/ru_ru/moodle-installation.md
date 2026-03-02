#### Загрузите плагин

Скачайте ZIP с последним выпуском из <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">репозитория FastComments Moodle на GitHub</a>.

#### Распакуйте в каталог вашего Moodle

Распакуйте ZIP в установку Moodle так, чтобы плагин находился по пути `<moodle-root>/local/fastcomments`. Директория плагина должна непосредственно содержать `version.php`, `lib.php` и другие файлы плагина (не вложенные в подпапку).

Например:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Установка через администратора Moodle

Войдите как администратор сайта и перейдите в **Администрирование сайта > Уведомления**. Moodle обнаружит новый плагин и предложит запустить установку.

#### Настройка плагина

После установки перейдите в **Администрирование сайта > Плагины > Локальные плагины > FastComments**, чтобы ввести настройки. См. раздел [Конфигурация](#moodle-configuration) для подробностей по каждой опции.