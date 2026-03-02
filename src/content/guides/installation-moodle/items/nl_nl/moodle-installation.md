#### Download de plugin

Download het nieuwste release-ZIP van de <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub-repository</a>.

#### Pak uit in je Moodle-map

Pak het ZIP-bestand uit in je Moodle-installatie zodat de plugin staat op `<moodle-root>/local/fastcomments`. De plugindirectory moet direct `version.php`, `lib.php` en andere pluginbestanden bevatten (niet genest in een submap).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Installeren via sitebeheer

Log in als sitebeheerder en ga naar **Sitebeheer > Meldingen**. Moodle detecteert de nieuwe plugin en vraagt je om de installatie uit te voeren.

#### Plugin configureren

Na installatie ga je naar **Sitebeheer > Plug-ins > Lokale plug-ins > FastComments** om je instellingen in te voeren. Zie de [Configuratie](#items-moodle-configuration) sectie voor details over elke optie.