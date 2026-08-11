---
Mentre il supporto di FastComments può aiutare con le migrazioni, la maggior parte può essere eseguita e monitorata facilmente senza alcun intervento del personale di supporto.

Supportiamo nativamente l'importazione di esportazioni dai seguenti fornitori:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

Navigando [qui](https://fastcomments.com/auth/my-account/manage-data/import) possiamo caricare il file contenente i dati da migrare.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='Pagina di importazione di FastComments con la selezione del provider e i campi di caricamento del file per un file di esportazione'; title='Il modulo della pagina di importazione' app-screenshot-end]

### Monitoraggio delle importazioni

FastComments utilizza un sistema di elaborazione dei job per gestire importazioni ed esportazioni. Una volta che il sistema ha preso in carico il tuo job, riporterà periodicamente lo stato del job nell'interfaccia di importazione o esportazione.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Pagina di importazione che mostra un job di importazione in esecuzione e lo stato segnalato dal sistema di elaborazione dei job'; title='Stato del job di importazione' app-screenshot-end]

Nota che lo stato delle importazioni e delle esportazioni è visibile a tutti gli amministratori dell'account.

Se il tuo job fallisce, non verrà riavviato automaticamente. L'importazione dovrà essere tentata nuovamente. Se una qualsiasi importazione o esportazione fallisce, gli amministratori del nostro sistema vengono notificati automaticamente. Se identifichiamo un problema, ti contatteremo per vedere se possiamo aiutare.

### Rieseguire l'importazione

Durante alcune migrazioni, è necessario eseguire l'importazione più volte. Ad esempio, è comune effettuare una prima migrazione di prova e poi eseguire nuovamente l'importazione con i dati più recenti prima di attivare il cambiamento.

Reimportare lo stesso contenuto **non creerà duplicati**.

### Sicurezza dei dati e scadenza

I file di importazione non sono accessibili tramite richieste esterne in alcun modo, e i file di importazione vengono eliminati dal nostro sistema non appena l'importazione è completata.