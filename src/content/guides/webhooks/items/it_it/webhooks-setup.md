---
Segui gli stessi passaggi per `localhost` come faresti in produzione. Assicurati di aver configurato i domini di produzione e i Segreti API.

Prima, vai alla [Amministrazione Webhook](https://fastcomments.com/auth/my-account/manage-data/webhooks). È accessibile tramite Manage Data -> Webhooks.

La pagina elenca tutti i webhook del tuo account:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Pagina di amministrazione dei Webhook che elenca ogni webhook con il suo URL, evento, dominio, metodo, stato e conteggio degli eventi in coda'; title='Elenco Webhook'; cacheBuster = 'v4' app-screenshot-end]

Fai clic su **New Webhook** per aggiungerne uno. Ogni webhook ha un URL, un evento di commento (creato, aggiornato o eliminato), un dominio e un metodo HTTP:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='Modulo per nuovo webhook con campi URL, evento, dominio e metodo HTTP più Invia Payload di Test'; title='Nuovo Webhook'; cacheBuster = 'v4' app-screenshot-end]

Ogni webhook viene consegnato in modo indipendente. Puoi inviare lo stesso evento a più endpoint, e un webhook con ambito **All Domains** riceve commenti da tutti i domini anche quando esiste un webhook specifico per dominio per lo stesso evento. Lo stesso URL, evento e dominio non possono essere aggiunti due volte.

Prima di salvare, fai clic su **Send Test Payload** per verificare che l'endpoint accetti una richiesta firmata. Consulta la sezione successiva, "Testing", per i dettagli.

Dall'elenco puoi modificare, disabilitare, riabilitare o eliminare un webhook. La disabilitazione mantiene gli eventi in coda fino a quando il webhook non viene riabilitato; l'eliminazione li scarta.

I webhook possono anche essere creati tramite l'API, ad esempio con Zapier. Questi appaiono nello stesso elenco con la fonte **API**. Consulta Managing Webhooks via the API.

---