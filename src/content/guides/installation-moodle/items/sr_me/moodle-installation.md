#### Преузмите додатак

Преузмите најновију ZIP архиву из <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub репозиторијума</a>.

#### Распакујте у ваш Moodle директоријум

Распакујте ZIP у вашу Moodle инсталацију тако да додатак буде на `<moodle-root>/local/fastcomments`. Директоријум додатка треба да садржи `version.php`, `lib.php`, и друге датотеке додатка директно (не уложене у поддиректоријум).

На пример:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Инсталирајте преко Moodle администратора

Пријавите се као администратор сајта и идите на **Администрација сајта > Обавештења**. Moodle ће открити нови додатак и затражити да покренете инсталацију.

#### Конфигуришите додатак

Након инсталације, идите на **Администрација сајта > Додаци > Локални додаци > FastComments** да унесете своја подешавања. Погледајте одељак [Конфигурација](#moodle-configuration) за детаље о свакој опцији.