---
Lo script per ogni estensione viene recuperato e invocato prima che il widget dei commenti inizi a recuperare il primo set di commenti e a renderizzare l'interfaccia utente.

Al caricamento iniziale, i seguenti dati verranno aggiunti all'oggetto dell'estensione:

- `config` - Un riferimento all'oggetto `config`.
- `translations` - Un riferimento all'oggetto `translations`.
- `commentsById` - Un riferimento a tutti i commenti per id.
- `root` - Un riferimento al nodo DOM radice.

Le estensioni dovrebbero sovrascrivere le funzioni desiderate, che il widget dei commenti invocherà nei momenti appropriati.

---