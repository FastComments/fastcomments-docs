Si attiva quando un utente pubblica il suo primo commento su questo sito (il tuo tenant). Questo è **una sola volta per utente** - i commenti successivi dello stesso utente non lo riattivano.

### Contesto che riceve l'agente

- Il nuovo commento.
- Eventuale contesto di thread / cronologia utente / pagina come configurato.

Quando il contesto della cronologia utente è attivato, l'elenco dei commenti recenti dell'utente sarà ovviamente vuoto (o conterrà solo questo), ma il fattore di fiducia e l'età dell'account sono valorizzati.

### Da notare

- "Primo commento su questo sito" è limitato al **tenant**, non a livello di sito in tutta FastComments. Un utente con commenti su altri siti FastComments attiverà comunque questo trigger la prima volta che pubblica sul tuo.
- Il trigger si attiva solo per gli utenti con un userId. I commenti anonimi non verificati senza un userId stabile non lo attivano.
- Il trigger si attiva quando il commento è approvato/visibile (non al momento della pubblicazione iniziale). Le modifiche o le azioni dei moderatori sul primo commento non lo riattivano.

### Usi comuni

- **Saluto di benvenuto** - il [modello Welcome Greeter](#template-welcome-greeter) è costruito attorno a questo trigger.
- **Onboarding** - invia un [DM warning](#tool-warn-user) (usato qui come un gentile avviso piuttosto che una ammonizione) indicando all'utente le linee guida della community.
- **Notifica al revisore** - se vuoi che una persona esamini il primo post di ogni nuovo commentatore, [`mark_comment_reviewed`](#tools-overview) può contrassegnarli per la revisione.

---