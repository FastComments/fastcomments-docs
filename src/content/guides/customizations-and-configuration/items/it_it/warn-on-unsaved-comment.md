[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, se un utente digita un commento e poi aggiorna la pagina, chiude la scheda o naviga altrove prima di inviarlo, la bozza viene persa silenziosamente.

Impostare **warnOnUnsavedComment** su true fa sì che il browser chieda all'utente di confermare prima di lasciare la pagina finché qualsiasi casella di commento, o una modifica in corso, contiene ancora del testo. Una volta inviato il commento il testo viene cancellato, quindi non viene mostrato alcun avviso.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Avviso su commento non salvato'; code-example-end]

L'avviso utilizza la finestra di dialogo nativa del browser. I browser moderni mostrano il proprio testo e ignorano il testo personalizzato, quindi il messaggio non può essere modificato.

Questa opzione carica una piccola estensione su richiesta, quindi non aggiunge nulla al widget per i siti che non la abilitano.