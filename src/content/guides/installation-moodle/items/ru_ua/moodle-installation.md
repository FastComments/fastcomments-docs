#### Скачайте плагин

Скачайте ZIP последнего релиза из <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">репозитория FastComments Moodle на GitHub</a>.

#### Распакуйте в каталог Moodle

Распакуйте ZIP в установку Moodle так, чтобы плагин располагался по адресу `<moodle-root>/local/fastcomments`. Директория плагина должна содержать `version.php`, `lib.php`, и другие файлы плагина непосредственно (не вложенными в подпапку).

Например:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Установка через админку Moodle

Войдите как администратор сайта и перейдите в **Администрирование сайта > Уведомления**. Moodle обнаружит новый плагин и предложит запустить установку.

#### Настройка плагина

После установки перейдите в **Администрирование сайта > Плагины > Локальные плагины > FastComments**, чтобы ввести ваши настройки. См. раздел [Конфигурация](#moodle-configuration) для подробностей по каждой опции.

---