#### Scarica il plugin

Scarica l'ultima release ZIP dal <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">repository FastComments Moodle su GitHub</a>.

#### Estrai nella tua directory di Moodle

Estrai lo ZIP nella tua installazione di Moodle in modo che il plugin risieda in `<moodle-root>/local/fastcomments`. La directory del plugin dovrebbe contenere `version.php`, `lib.php`, e altri file del plugin direttamente (non annidati in una sottocartella).

Per esempio:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Installa tramite l'amministrazione di Moodle

Accedi come amministratore del sito e vai su **Amministrazione del sito > Notifiche**. Moodle rileverà il nuovo plugin e ti chiederà di avviare l'installazione.

#### Configura il plugin

Dopo l'installazione, vai su **Amministrazione del sito > Plugin > Plugin locali > FastComments** per inserire le impostazioni. Consulta la sezione [Configurazione](#moodle-configuration) per i dettagli su ogni opzione.

---