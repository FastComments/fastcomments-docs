È possibile vietare gli utenti che utilizzano determinati provider di posta elettronica usando i caratteri jolly.

Ad esempio, se scopri che tutti i commenti provenienti da **@bademail.com** sono spam, puoi semplicemente vietare l'intero provider di posta elettronica inserendo "*@bademail.com" nel campo email quando aggiungi un utente bloccato.

Nota il "*" prima del @ nell'indirizzo email.

### Sottodomini

Un divieto a livello di dominio copre anche tutti i sottodomini di quel dominio. Vietare `*@bademail.com` vieta anche `someone@mail.bademail.com` e `someone@eu.mail.bademail.com`, quindi non è necessario aggiungere un divieto separato per ogni sottodominio.

Se desideri vietare solo un sottodominio specifico, inserisci quel sottodominio, ad esempio `*@mail.bademail.com`. Tale divieto non influisce su `someone@bademail.com`.

### Vietare un dominio da un commento

Non è necessario digitare manualmente il modello. Quando vieti un utente da un commento nella pagina Modera commenti, la finestra di dialogo di blocco ha una casella di controllo "Ban All @domain Users" che crea lo stesso divieto `*@domain` per il dominio email del commentatore.

### Modelli supportati

L'unica forma di carattere jolly supportata è un singolo `*` al posto dell'intera parte del nome, seguito da `@` e un dominio. Altre forme vengono rifiutate quando si tenta di salvarle:

- `*@*.bademail.com` non è necessario, perché `*@bademail.com` copre già i sottodomini.
- `name*@bademail.com` e `*bademail.com` non sono supportati.

---