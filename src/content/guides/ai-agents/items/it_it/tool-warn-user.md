Lo strumento Warn invia un avviso privato tramite DM a un utente riguardo a un commento specifico e allo stesso tempo registra l'avviso nella [memoria dell'agente](#agent-memory-system). Le due scritture sono atomiche: l'utente non vede mai un avviso che non sia anche registrato.

### Perché esiste

La politica di escalation della piattaforma è **prima l'avviso, poi il ban solo se l'utente recidiva**. Lo strumento Warn rende applicabile questa politica: dà all'utente la possibilità di correggere il comportamento, e il record dell'avviso è ciò che un agente futuro troverà quando cercherà nella memoria prima di prendere in considerazione un ban.

Lo strumento evita anche duplicazioni: se l'agente ha già emesso un avviso allo stesso utente per lo stesso commento, un secondo avviso è un'operazione nulla. Quindi un LLM che va in loop o riattiva lo stesso commento non può mandare all'utente più avvisi ripetuti.

### Cosa contiene l'avviso

Un messaggio breve (limitato a 1000 caratteri) mostrato all'utente come DM. Gli avvisi efficaci sono:

- **Specifici** - "Gli attacchi personali verso utenti nominati non sono consentiti in questa community" è meglio di "il tuo commento è stato segnalato."
- **Brevi** - poche frasi al massimo.
- **Azione concrete** - dì all'utente cosa modificare. "Per favore modifica il tuo commento per rimuovere il nome dell'utente, altrimenti verrà rimosso."

Non scrivi tu il messaggio; lo fa l'agente, basandosi sul [prompt iniziale](#personality-prompt) e sulle [linee guida della community](#community-guidelines). Il tuo compito è scrivere un prompt che produca avvisi efficaci.

### Quando consentirne l'uso

Per qualsiasi agente di tipo moderazione. Il template Moderator lo abilita per impostazione predefinita.

### Approvazioni

Meno frequentemente soggetto ad autorizzazione rispetto a [Ban user](#tool-ban-user). Vale la pena richiedere l'approvazione durante le prime settimane di vita di un agente in modo da individuare avvisi inappropriati prima che vengano inviati, ma la maggior parte degli operatori rimuove il vincolo una volta che l'agente produce output affidabile.

### Vedi anche

- [Ban user](#tool-ban-user) - il passo successivo nell'escalation.
- [Agent Memory System](#agent-memory-system) - dove risiedono i record degli avvisi.