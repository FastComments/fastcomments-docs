#### Завантажте плагін

Завантажте останній ZIP-реліз із <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">репозиторію FastComments Moodle на GitHub</a>.

#### Розпакуйте в каталог вашого Moodle

Розпакуйте ZIP у вашу інсталяцію Moodle так, щоб плагін знаходився за шляхом `<moodle-root>/local/fastcomments`. Директорія плагіна повинна містити `version.php`, `lib.php`, та інші файли плагіна безпосередньо (не в підпапці).

Наприклад:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Встановлення через адмінку Moodle

Увійдіть як адміністратор сайту та перейдіть до **Адміністрування сайту > Сповіщення**. Moodle виявить новий плагін і запропонує запустити встановлення.

#### Налаштування плагіна

Після встановлення перейдіть до **Адміністрування сайту > Плагіни > Локальні плагіни > FastComments**, щоб ввести свої налаштування. Див. розділ [Налаштування](#items-moodle-configuration) для детальної інформації про кожну опцію.

---