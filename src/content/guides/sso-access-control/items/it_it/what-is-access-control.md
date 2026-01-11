Con FastComments SSO Access Control, talvolta indicato come RBAC, gli utenti possono essere limitati ad accedere solo a determinate pagine o thread di commenti. Inoltre, gli utenti possono `@mention`arsi solo tra loro nello stesso gruppo.

## In dettaglio

Possiamo inserire `Users` e opzionalmente `Pages` in gruppi.

Quando i `Users` sono inseriti in gruppi, e l'opzione `Limit Comments by SSO User Groups` è abilitata nelle impostazioni del widget, allora gli utenti vedranno solo i commenti degli utenti nel loro stesso gruppo e potranno `@mention`are solo utenti nello stesso gruppo.

Inoltre, le `Pages` possono essere inserite in gruppi, e allora gli utenti possono accedere ai commenti solo per le pagine a cui hanno accesso.

Li chiamiamo gruppi "User-Level" rispetto ai gruppi "Page-Level".

Quindi quale è quello giusto per te?

#### Usa i gruppi User-Level se...

- Vuoi usare lo stesso valore `urlId` (URL della pagina, o ID dell'articolo), ma limitare i commenti per gruppo.
- Per esempio, vuoi avere i gruppi "New User" e "Veteran User", e non dovrebbero mai vedere i commenti l'uno dell'altro sulle stesse pagine.

#### Usa i gruppi Page-Level se...

- I tuoi gruppi hanno pagine specifiche.
- Per esempio, gli utenti nel gruppo "Public Pages" non dovrebbero mai visualizzare gli articoli "Top Secret".