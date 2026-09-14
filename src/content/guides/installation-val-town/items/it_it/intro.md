[Val Town](https://val.town) esegue TypeScript su Deno, quindi un val è un server reale. Questo lo rende adatto a FastComments: il widget è un tag script nella pagina, e tutto ciò che richiede un segreto, come Secure SSO o la verifica di un webhook, può essere eseguito lato server nello stesso val.

Questa guida copre l'aggiunta del widget dei commenti a un val HTTP, la visualizzazione dei conteggi dei commenti su una pagina indice, l'accesso degli utenti con l'account Val Town che già possiedono, e la ricezione dei webhook dei commenti.

Non è necessario un account per provarlo. Gli esempi usano `tenantId: "demo"`, un sandbox condiviso, e il Passo 2 spiega come passare al proprio.