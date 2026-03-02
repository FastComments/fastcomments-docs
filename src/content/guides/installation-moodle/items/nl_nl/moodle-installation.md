#### Download de plugin

Download de nieuwste release ZIP van de <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub repository</a>.

#### Pak uit naar uw Moodle-directory

Pak de ZIP uit in uw Moodle-installatie zodat de plugin zich bevindt op `<moodle-root>/local/fastcomments`. De plugindirectory moet rechtstreeks `version.php`, `lib.php` en andere pluginbestanden bevatten (niet genest in een submap).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Installeren via Moodle-beheer

Meld u aan als sitebeheerder en navigeer naar **Sitebeheer > Meldingen**. Moodle detecteert de nieuwe plugin en vraagt u de installatie uit te voeren.

#### Configureer de plugin

Na installatie, ga naar **Sitebeheer > Plugins > Lokale plugins > FastComments** om uw instellingen in te voeren. Zie de [Configuratie](#moodle-configuration) sectie voor details over elke optie.