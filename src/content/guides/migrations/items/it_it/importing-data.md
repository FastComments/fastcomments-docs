Sebbene il Supporto FastComments possa aiutare con le migrazioni, la maggior parte può essere eseguita e monitorata facilmente senza l'intervento del personale di supporto.

Supportiamo nativamente l'importazione di esportazioni dai seguenti provider:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (tramite il plugin)

Navigando [qui](https://fastcomments.com/auth/my-account/manage-data/import) possiamo caricare il file contenente i dati da migrare.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitoraggio delle importazioni

FastComments utilizza un sistema di elaborazione job per il trattamento delle importazioni e delle esportazioni. Una volta che il sistema ha preso in carico il tuo job, riporterà periodicamente lo stato del job nell'interfaccia di importazione o esportazione.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Nota che lo stato delle Importazioni e delle Esportazioni è visualizzabile da tutti gli amministratori dell'account.

Se il tuo job fallisce, non verrà riavviato automaticamente. L'importazione dovrà essere tentata nuovamente. Se qualsiasi importazione o esportazione fallisce, i nostri amministratori di sistema vengono automaticamente notificati. Se identifichiamo un problema, ti contatteremo per vedere se possiamo aiutare.

### Riesecuzione dell'importazione

Durante alcune migrazioni, è necessario eseguire l'importazione più volte. Per esempio, è comune fare una prima migrazione di prova, e poi eseguire nuovamente l'importazione con i dati più recenti prima di effettuare il passaggio definitivo.

Reimportare lo stesso contenuto **non creerà duplicati**.

### Sicurezza dei dati e scadenza

I file di importazione non sono accessibili tramite richieste esterne in alcun modo, e i file di importazione vengono eliminati dal nostro sistema non appena l'importazione è completata.