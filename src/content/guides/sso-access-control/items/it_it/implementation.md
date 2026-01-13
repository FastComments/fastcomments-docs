#### Menzionare utenti in altri gruppi

Se due utenti appartengono a due diversi insiemi di gruppi e non c'è intersezione, non potranno `@mention`arsi a vicenda.

Se un utente digita manualmente un `@mention` e invia il suo commento, rimarrà testo semplice. L'altro utente non verrà taggato.

#### Gestione dei gruppi

`Groups` sono definiti utilizzando rispettivamente le risorse API `Pages` e `SSOUsers`.

L'API `Pages` può essere invocata per definire l'insieme di gruppi autorizzati ad accedere alla pagina. Per impostazione predefinita, tutti i gruppi e gli utenti che non appartengono a un gruppo hanno accesso.

Allo stesso modo, l'API `SSOUsers` può essere invocata per definire i gruppi associati a ciascun utente.

Per entrambe le risorse, non ci sono limitazioni su quando i gruppi possono essere impostati o aggiornati.

Se si desidera soltanto impedire che gli utenti si `@mention`ino a vicenda, allora non è necessario prendere in considerazione le `Pages`.

##### Nota!

La definizione e l'aggiornamento dei gruppi utente SSO non richiede l'uso dell'API e può invece essere aggiornato automaticamente definendo gli id dei gruppi nel payload SSO passato al widget dei commenti. Tuttavia, per elenchi estesi di gruppi, questo non è raccomandato poiché l'utente dovrebbe inviare questo payload ad ogni caricamento della pagina.