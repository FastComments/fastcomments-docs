[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, i nuovi commenti in tempo reale appaiono in cima all'elenco dei commenti man mano che vengono pubblicati.

Quando questa opzione è abilitata, i nuovi commenti in tempo reale verranno aggiunti invece in fondo all'elenco. Questo influisce su come i commenti appaiono quando vengono pubblicati in diretta mentre gli utenti visualizzano la discussione.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Con questa impostazione abilitata:
- I nuovi commenti in tempo reale pubblicati da altri utenti appariranno in fondo all'elenco dei commenti
- Gli utenti vedranno i nuovi commenti apparire sotto i commenti esistenti in tempo reale
- Questo influisce solo sugli aggiornamenti dei commenti in tempo reale - non sul caricamento iniziale della pagina
- Questo può aiutare a mantenere il flusso di lettura quando gli utenti seguono una discussione

Nota che questa impostazione influisce solo su dove vengono posizionati i nuovi commenti in tempo reale mentre arrivano. Non influenza l'ordinamento iniziale quando la pagina viene caricata.