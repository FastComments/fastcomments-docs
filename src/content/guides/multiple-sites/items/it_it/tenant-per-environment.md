È comune avere un sub-tenant per ogni ambiente di test o di sviluppo con FastComments. Ogni tenant avrebbe la propria configurazione, i propri dati e le proprie chiavi API. Configurazione, dati e utenti non possono essere condivisi tra tenant.
Tutto è isolato. Tuttavia, i super admin del tenant principale possono impersonare gli utenti nei tenant figli.

Ci sono due approcci:

- Il tenant principale è per la produzione, e i sub-tenant sono per gli ambienti di test.
- Il tenant principale è semplicemente per la fatturazione, e ogni sub-tenant è per prod, test, e così via.

La prima opzione è generalmente più facile da comprendere per gli utenti, ma questo può dipendere dalla vostra organizzazione.

I tenant possono essere creati [here](https://eu.fastcomments.com/auth/my-account/tenants) se avete il pacchetto. È anche qui che i super admin impersonerebbero gli utenti. I tenant possono anche essere creati tramite l'API per configurazioni più personalizzate/automatizzate.

Indipendentemente dall'approccio adottato, dovrete aggiungere i moderatori e gli utenti che vogliono vedere i dati di produzione nel tenant "prod". Quindi, per esempio, se volete procedere con l'opzione B e usare il tenant principale per la fatturazione, e avere un sub-tenant per "prod", dovrete aggiungere il tenant, passare al nuovo tenant e aggiungere i vostri utenti amministratori e moderatori per il sub-tenant. 

Infine, per chiarezza, la pagina Modera commenti sarà vuota con l'opzione B per il tenant principale.