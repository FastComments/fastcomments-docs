**Usi Moodle?** Pubbliciamo anche un plugin dedicato per Moodle per FastComments con un'integrazione più stretta rispetto a LTI 1.3 (hook di sincronizzazione dei voti, report attività più approfonditi, interfaccia delle impostazioni nativa di Moodle). Vedi la <a href="/guide-installation-moodle.html" target="_blank">guida all'installazione del plugin Moodle</a>. Il flusso LTI 1.3 qui sotto è la scelta giusta se vuoi una registrazione unica che copra anche altri LMS, o se l'amministratore di Moodle non installerà plugin di terze parti.

Moodle 4.0+ supporta la Registrazione Dinamica LTI 1.3 tramite il plugin External Tool.

#### Apri la schermata di gestione dello strumento

1. Accedi a Moodle come amministratore del sito.
2. Naviga su **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Incolla l'URL

Vedrai una scheda etichettata **URL dello strumento**. Incolla l'URL di registrazione di FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottienilo qui</a>) nel campo di testo e clicca **Aggiungi LTI Advantage**.

Moodle aprirà una schermata di registrazione che mostra l'identità dello strumento e le autorizzazioni richieste. Verifica e clicca **Attiva** (o **Registra**, a seconda della versione di Moodle).

La finestra popup si chiude al termine della registrazione; il nuovo strumento FastComments appare nella lista **Strumenti** con lo stato **Attivo**.

#### Rendilo disponibile

Per impostazione predefinita Moodle aggiunge i nuovi strumenti alla lista "strumenti del corso" ma non li mostra nel selettore di attività. Per rendere FastComments disponibile in tutto il corso:

1. Clicca l'icona dell'ingranaggio sulla scheda FastComments.
2. Sotto **Tool configuration usage**, scegli **Mostra nel selettore attività e come strumento preconfigurato**.
3. Salva.

Gli insegnanti ora possono aggiungere FastComments a qualsiasi corso tramite **Aggiungi un'attività o una risorsa** > **FastComments**.