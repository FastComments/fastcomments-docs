#### Преузмите додатак

Преузмите најновији ZIP издање са <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub репозиторијума</a>.

#### Распакујте у ваш Moodle директоријум

Распакујте ZIP у вашу Moodle инсталацију тако да додатак буде у `<moodle-root>/local/fastcomments`. Директоријум додатка треба да садржи `version.php`, `lib.php`, и друге датотеке додатка директно (не унутар поддиректоријума).

На пример:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Инсталирање преко Moodle администратора

Пријавите се као администратор сајта и идите на **Site Administration > Notifications**. Moodle ће открити нови додатак и упитати вас да покренете инсталацију.

#### Конфигурисање додатка

Након инсталације, идите на **Site Administration > Plugins > Local plugins > FastComments** да унесете ваша подешавања. Погледајте одељак [Configuration](#moodle-configuration) за детаље о свакој опцији.