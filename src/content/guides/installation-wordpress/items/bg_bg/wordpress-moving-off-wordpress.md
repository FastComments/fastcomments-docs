Ако преминавате вашия сайт от WordPress и искате FastComments на новия сайт, не ви е необходим плъгинът за WordPress. Експортирайте вашите коментари от WordPress, след което качете файла на [Import page](https://fastcomments.com/auth/my-account/manage-data/import) в таблото на FastComments.

Поддържаме два формата за експортиране от WordPress.

### WordPress XML (Recommended)

Това е файлът от вградената в WordPress функция за експортиране, така че не е необходим допълнителен плъгин.

1. В админ панела на WordPress отидете на `Tools -> Export`.
2. Изберете `All content` и кликнете върху `Download Export File`.
3. На FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import) изберете `WordPress (.xml)` и качете файла.

Всеки коментар е свързан с URL адреса на публикацията, в която е оставен, което вече е включено във файла.

Импортът запазва името на автора, имейла и уебсайта, датата, съдържанието, нишката от отговори и дали коментарът е одобрен. Аватарите на коментаторите се пренасят от Gravatar. Гласовете не са част от този формат.

### WordPress CSV

Това е файлът от [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Инсталирайте плъгина в админ панела на WordPress и експортирайте вашите коментари като CSV.
2. Заменете всяка стойност `comment_post_ID` с URL адреса на публикацията.
3. На FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import) изберете `WordPress (.csv)` и качете файла.

Всеки коментар е свързан със колоната `comment_post_ID`. WordPress попълва тази колона с идентификатора на публикацията, а вашият нов сайт няма WordPress идентификатори за публикации, затова стъпка 2 я заменя с URL адреса.

Импортът запазва името на автора, имейла и уебсайта, датата, съдържанието, нишката от отговори и дали коментарът е одобрен. Аватарите на коментаторите се пренасят от Gravatar. Също така се запазва флагът за спам на WordPress и харесванията/нехаресванията от wpDiscuz, ако файлът ги съдържа.

### Matching Comments to Your New Pages

Ако вашият нов сайт запази същите URL адреси като вашия WordPress сайт, коментарите ще се появят на съответните страници без допълнителна настройка.

Ако домейнът се промени, стартирайте [Domain Migration tool](/guide-migrations.html#migrating-domains) след импорта. Ако индивидуалните URL адреси на страниците се променят, можете да [migrate each page](/guide-migrations.html#migrating-pages) от стария URL към новия.

За масови миграции на страници, като премахване на домейна от стойността, която подавате в полето за коментари [urlId](/guide-customizations-and-configuration.html#url-id), [open a support ticket](https://fastcomments.com/auth/my-account/help) и ние ще се погрижим за това.

### Before You Switch

Можете да изпълнявате импорта колкото пъти желаете. Преимпортирането на същия файл [не създава дублирани записи](/guide-migrations.html#importing-data), така че можете да импортирате веднъж, за да тествате новия сайт, след което да импортирате отново с последните си коментари точно преди да преминете.

За файлове за експортиране, по-големи от 1 GB, [свържете се с поддръжката](https://fastcomments.com/auth/my-account/help).

За да добавите FastComments към вашия нов сайт, вижте [Ръководство за инсталиране](/guide-installation.html).