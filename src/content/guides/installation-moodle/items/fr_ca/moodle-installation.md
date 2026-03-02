#### Télécharger le plugin

Download the latest release ZIP from the <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">dépôt GitHub de FastComments Moodle</a>.

#### Extraire dans votre répertoire Moodle

Extrayez le ZIP dans votre installation Moodle de sorte que le plugin se trouve à `<moodle-root>/local/fastcomments`. Le répertoire du plugin devrait contenir `version.php`, `lib.php`, et d'autres fichiers du plugin directement (pas imbriqués dans un sous-dossier).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Installer via l'administration de Moodle

Connectez-vous en tant qu'administrateur du site et rendez-vous dans **Administration du site > Notifications**. Moodle détectera le nouveau plugin et vous invitera à lancer l'installation.

#### Configurer le plugin

Après l'installation, allez à **Administration du site > Plugins > Plugins locaux > FastComments** pour entrer vos paramètres. Consultez la [Configuration](#moodle-configuration) section pour les détails sur chaque option.