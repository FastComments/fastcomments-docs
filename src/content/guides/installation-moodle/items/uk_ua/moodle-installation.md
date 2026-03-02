#### Завантажте плагін

Завантажте останній ZIP-реліз із <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">репозиторію FastComments Moodle на GitHub</a>.

#### Розпакуйте у вашу директорію Moodle

Розпакуйте ZIP у вашу установку Moodle так, щоб плагін знаходився за шляхом `<moodle-root>/local/fastcomments`. Каталог плагіна повинен містити `version.php`, `lib.php` та інші файли плагіна безпосередньо (не розміщено в підпапці).

Наприклад:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Встановлення через адмін-панель Moodle

Увійдіть як адміністратор сайту та перейдіть до **Адміністрування сайту > Сповіщення**. Moodle виявить новий плагін і запропонує запустити встановлення.

#### Налаштування плагіна

Після встановлення перейдіть до **Адміністрування сайту > Плагіни > Локальні плагіни > FastComments**, щоб ввести свої налаштування. Див. розділ [Налаштування](#moodle-configuration) для детальної інформації про кожну опцію.