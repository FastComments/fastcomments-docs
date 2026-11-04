Per impostazione predefinita, gli utenti possono eliminare i propri commenti. Inoltre, eliminando il proprio commento vengono eliminati automaticamente
tutti i commenti figli e transitori nel thread. Questo comportamento è attivo anche in tempo reale.

Puoi limitare questo comportamento nei seguenti modi:

- In alternativa, anonimizzare il commento eliminato (impostare nome e testo su `[deleted]` o su un valore personalizzato).
- Non consentire l'eliminazione dei commenti quando ci sono risposte. Viene mostrato un messaggio di errore personalizzabile.
- Limitare l'eliminazione quando un commento ha risposte solo agli amministratori e ai moderatori.

Questo può essere configurato tramite la sezione `Comment Thread Deletion` nell'interfaccia di personalizzazione del Widget.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; alt='Opzioni di eliminazione del thread di commenti nell\'interfaccia di personalizzazione del widget per anonimizzare o limitare le eliminazioni con risposte'; title='Personalizza il comportamento di eliminazione per le risposte' app-screenshot-end]