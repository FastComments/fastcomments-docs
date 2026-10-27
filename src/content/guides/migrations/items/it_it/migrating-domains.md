FastComments fornisce un modo automatizzato per migrare i tuoi commenti tra domini.

La migrazione del dominio richiede semplicemente un dominio `from` e un dominio `to`.

Questo **sposta** i commenti, non li copia. Se desideri copiare i commenti, contattaci.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Strumento di migrazione dei domini con i campi da e a dominio e il conteggio dei commenti migrati'; title='Migrazione dei domini' app-screenshot-end]

Questo è utile, ad esempio, se parte della tua migrazione a FastComments prevede la migrazione da un provider diverso, quindi i dati di importazione dei commenti potrebbero contenere dati che devono essere migrati. In questo caso, puoi eseguire l'importazione e poi la migrazione del dominio.

### Monitoraggio del progresso

Lo strumento di migrazione dei domini utilizza lo stesso sistema di elaborazione dei job di FastComments degli altri strumenti di gestione dei dati.

Potrebbe esserci un ritardo prima che inizi la migrazione. Questo è normale, poiché il sistema controlla periodicamente la presenza di nuovi job da elaborare.

Man mano che il job viene eseguito, mostrerà il numero di commenti trovati da migrare e il numero di commenti migrati finora.

### Migrazione dei domini tramite API

I domini possono anche essere migrati con [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), impostando `type` su `domain`. Lo stato di avanzamento è disponibile tramite [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), e la migrazione può essere annullata per 90 giorni.

---