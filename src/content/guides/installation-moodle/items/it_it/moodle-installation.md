#### Download the Plugin

Scarica l'ultima release ZIP dal <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">repository GitHub di FastComments Moodle</a>.

#### Extract to Your Moodle Directory

Estrai lo ZIP nella tua installazione di Moodle in modo che il plugin si trovi in `<moodle-root>/local/fastcomments`. La directory del plugin dovrebbe contenere `version.php`, `lib.php`, e altri file del plugin direttamente (non annidati in una sottocartella).

Per esempio:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Install via Moodle Admin

Accedi come amministratore del sito e vai su **Amministrazione del sito > Notifiche**. Moodle rileverà il nuovo plugin e ti chiederà di eseguire l'installazione.

#### Configure the Plugin

Dopo l'installazione, vai su **Amministrazione del sito > Plugin > Plugin locali > FastComments** per inserire le tue impostazioni. Consulta la sezione [Configurazione](#moodle-configuration) per i dettagli su ciascuna opzione.