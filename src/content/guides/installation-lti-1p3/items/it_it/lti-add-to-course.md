Una volta che FastComments è registrato nel tuo LMS, gli istruttori lo aggiungono ai corsi allo stesso modo in cui aggiungono qualsiasi altro strumento esterno LTI.

#### D2L Brightspace

Nell'area dei contenuti di un corso:

1. Clicca **Aggiungi attività esistenti** > **Strumenti di apprendimento esterni**.
2. Seleziona **FastComments** dall'elenco.
3. Lo strumento appare come un argomento nell'area dei contenuti. Aprilo una volta per inizializzare il thread di commenti per quella risorsa.

#### Moodle

In un corso:

1. Attiva la **Modalità di modifica**.
2. Nella sezione dove vuoi i commenti, clicca **Aggiungi un'attività o una risorsa**.
3. Scegli **FastComments** dal selettore delle attività.
4. Salva. Gli studenti vedono il thread di commenti incorporato nella sezione.

#### Blackboard Learn

In un corso:

1. Vai in un'area dei contenuti.
2. Clicca **Crea contenuti** > **FastComments** (sotto "Strumenti di apprendimento").
3. Configura un nome e invia.

#### Sakai

I manutentori del sito aggiungono lo strumento tramite **Site Info** > **Manage Tools** > scorri fino a **External Tools** > seleziona **FastComments** > **Continue**.

#### Come vengono definiti gli ambiti dei thread

FastComments crea un thread di commenti separato per **(LMS instance, course, resource link)**. Ciò significa:

- Due corsi diversi nello stesso LMS ottengono thread separati, anche se usano lo stesso nome dello strumento.
- Lo stesso strumento FastComments usato in due punti all'interno di un corso crea due thread.
- Due installazioni diverse di Brightspace sotto lo stesso account FastComments ottengono thread distinti - l'hostname del LMS fa parte dell'identificatore del thread.

#### SSO

Gli studenti non vedono una schermata di accesso. Il LMS invia la loro identità (nome, email, avatar, ruolo) a FastComments tramite il lancio LTI, e FastComments li autentica automaticamente. I loro commenti sono attribuiti al loro account LMS.

Gli utenti con i ruoli LMS **Instructor** o **Administrator** vengono mappati automaticamente ai ruoli di moderatore/amministratore di FastComments per il thread.