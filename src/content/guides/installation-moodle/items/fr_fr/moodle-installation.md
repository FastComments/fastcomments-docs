---
#### Télécharger le plugin

Téléchargez le ZIP de la dernière version depuis le <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">dépôt GitHub FastComments Moodle</a>.

#### Extraire dans votre répertoire Moodle

Extrayez le ZIP dans votre installation Moodle de sorte que le plugin se trouve à `<moodle-root>/local/fastcomments`. Le répertoire du plugin doit contenir `version.php`, `lib.php`, et d'autres fichiers du plugin directement (pas imbriqués dans un sous-dossier).

Par exemple :

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Installer via l'administration de Moodle

Connectez-vous en tant qu'administrateur du site et naviguez vers **Administration du site > Notifications**. Moodle détectera le nouveau plugin et vous invitera à lancer l'installation.

#### Configurer le plugin

Après l'installation, allez dans **Administration du site > Plugins > Plugins locaux > FastComments** pour saisir vos paramètres. Consultez la section [Configuration](#moodle-configuration) pour les détails sur chaque option.

---