Una fattura Flex è un elenco di voci, una per ogni tipo di utilizzo, ognuna delle quali mostra la quantità e la tariffa, ad esempio `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. Le tariffe sono riportate nella [pagina dei prezzi](https://fastcomments.com/traffic-pricing) e nella pagina [Billing Info](https://fastcomments.com/auth/my-account/billing-info). Questa sezione spiega il significato di ogni quantità.

### Conteggiate Durante Il Mese

Queste si azzerano ogni mese e vengono conteggiate dal primo all'ultimo giorno del mese di calendario (UTC):

- **Page Loads**: il numero di volte in cui un widget o un feed di commenti è stato caricato sulle tue pagine. Ogni caricamento del widget dei commenti viene conteggiato, indipendentemente da quanti commenti mostra. Non vi è alcun addebito per thread di commenti o per pagina.
- **API Credits**: chiamate alla REST API con la tua chiave API.
- **Small Widgets Credits**: caricamenti dei widget piccoli, come i conteggi dei commenti e i badge.
- **LLM Tokens**: token utilizzati dagli [AI Agents](/guide-ai-agents.html) e da altre funzionalità LLM.

### Conteggiate Alla Fine Del Mese

Queste sono un'istantanea presa al momento della creazione della fattura:

- **Comments Stored**: il numero totale di commenti nel tuo account, non solo quelli aggiunti questo mese. L'eliminazione dei commenti lo riduce, tranne nel caso in cui un commento eliminato con risposte venga mantenuto come segnaposto affinché le risposte continuino ad avere senso.
- **SSO Users**: il numero totale di utenti SSO creati dal tuo sito, incluse persone che hanno commentato una sola volta molto tempo fa. Se questa è la voce più grande della tua fattura, chiedi al supporto la fatturazione in base agli utenti attivi mensili, dove vengono conteggiati solo gli utenti che hanno effettuato l'accesso durante il mese.
- **Moderators**, **Admins**, **SSO Moderators** e **SSO Admins**: postazioni, trattate in [Admin and Moderator Seats](#admin-and-moderator-seats). Il primo admin è gratuito.
- **Domains**: i siti nel tuo account. Il primo sito è gratuito e ogni sito aggiuntivo viene fatturato mensilmente, per tutto il tempo in cui rimane nell'account. Un sottodominio viene conteggiato solo se lo aggiungi come sito a sé stante; un singolo dominio di primo livello copre i suoi sottodomini. Vedi [Multiple Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: sotto-account gestiti da questo account, se presenti, più il loro utilizzo come voci separate.

### Minimo

Se le voci sommano a meno del minimo Flex, la fattura viene portata fino al minimo. Un piccolo sito con un admin e un dominio paga solo il minimo.

### Anteprima Della Fattura

La pagina [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) mostra ogni voce della prossima fattura con i numeri odierni, così non ci sono sorprese. I conteggi dei page load, delle API e dei widget piccoli possono essere in ritardo fino a un minuto; il resto è in tempo reale.

### Altre Valute

L'utilizzo Flex è prezzato in dollari USA. Se la valuta della tua fattura è diversa, ogni voce viene convertita al tasso di cambio del giorno in cui la fattura viene creata, quindi lo stesso utilizzo può costare importi leggermente diversi da mese a mese. Vedi [Currencies](#currencies).