#### Come Appaiono i Commenti nei Tuoi Corsi

Una volta abilitata l'integrazione LTI e installata l'App esterna, FastComments funziona automaticamente in base alle posizioni che hai configurato:

#### Visualizzazione compito

Se la posizione **Visualizzazione compito** è abilitata, i commenti appaiono automaticamente sotto ogni compito del corso. Studenti e istruttori vedono una sezione di commenti a thread quando visualizzano un compito — non è necessaria alcuna configurazione aggiuntiva per singolo compito.

Ogni compito ottiene il proprio thread di commenti separato.

#### Pulsante dell'Editor di contenuti ricchi

Se la posizione **Pulsante dell'Editor** è abilitata, gli istruttori possono incorporare FastComments in qualsiasi contenuto che utilizzi l'Editor di contenuti ricchi:

1. Modifica una **Pagina**, un **Quiz** o un **Annuncio**.
2. Nella barra degli strumenti dell'Editor di contenuti ricchi, clicca il pulsante **FastComments**.
3. FastComments viene incorporato automaticamente nel contenuto.
4. Salva la pagina.

Quando gli studenti visualizzano la pagina, il widget FastComments incorporato si carica con un thread di commenti unico per quella pagina.

#### SSO automatico

In entrambe le posizioni, gli studenti effettuano l'accesso automaticamente tramite il loro account Canvas. Nomi, email e avatar vengono sincronizzati tramite il lancio LTI, non è necessario un accesso separato.

#### Limitare l'accesso pubblico (consigliato)

Per impostazione predefinita, i dati dei commenti di FastComments sono leggibili pubblicamente. Chiunque riesca a indovinare l'URL di un thread o l'endpoint API può visualizzarne i commenti, anche al di fuori di Canvas. Per le discussioni del corso quasi certamente vorrai limitare la visualizzazione solo agli studenti iscritti.

Apri la tua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pagina di personalizzazione del widget</a> e crea una regola con **Require SSO To View Comments** abilitato, quindi imposta il livello di sicurezza su **Secure SSO** in modo che i thread possano essere caricati solo tramite il lancio LTI autenticato.

Vedi [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) per la procedura completa, inclusa la modalità per limitare la regola a un singolo dominio o pagina.