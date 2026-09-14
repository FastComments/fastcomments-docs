Quattro vals pubblici che puoi remixare, ognuno coprendo una parte di questa guida.

**[Blog con commenti](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) è un blog Markdown con un thread sotto ogni post e conteggi di commenti in blocco nella pagina indice. Funziona nel momento in cui lo remixi, e una variabile d'ambiente lo punta al tuo account.

**[Demo SSO](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) accede il visitatore con il suo account Val Town e passa quell'identità al widget, così non c'è un secondo login.

**[Ricevitore Webhook](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) verifica la firma HMAC su ogni consegna e memorizza gli eventi in SQLite. Ha un pulsante che firma un payload di test e lo invia a se stesso, così puoi vedere la verifica riuscire prima di configurare un webhook reale.

**[Competenze agente](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) è una libreria di competenze dell'agente FastComments che copre il widget, SSO, la REST API, la moderazione e la migrazione da Disqus. Remixala e l'agente di Val Town, Townie, carica automaticamente le competenze dalla cartella `skills/`, così il tuo agente sa come integrare i commenti senza che tu debba incollare la documentazione nella chat.

Le stesse competenze si installano ovunque altro con `npx skills add fastcomments/skills`.