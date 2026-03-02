#### Download the Plugin

Téléchargez le dernier fichier ZIP de la release depuis le <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">dépôt GitHub FastComments Moodle</a>.

#### Extract to Your Moodle Directory

Extrayez le ZIP dans votre installation Moodle de sorte que le plugin se trouve à `<moodle-root>/local/fastcomments`. Le répertoire du plugin doit contenir `version.php`, `lib.php`, et d'autres fichiers du plugin directement (pas imbriqués dans un sous-dossier).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Install via Moodle Admin

Connectez-vous en tant qu'administrateur du site et allez dans **Administration du site > Notifications**. Moodle détectera le nouveau plugin et vous invitera à lancer l'installation.

#### Configure the Plugin

Après l'installation, allez dans **Administration du site > Plugins > Plugins locaux > FastComments** pour saisir vos paramètres. Consultez la section [Configuration](#items-moodle-configuration) pour les détails sur chaque option.

---