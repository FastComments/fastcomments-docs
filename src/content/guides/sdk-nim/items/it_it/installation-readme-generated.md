### Utilizzo di Nimble

```bash
nimble install fastcomments
```

### Compilazione dal sorgente

```bash
nimble build
```

### Contenuto della libreria

Questa libreria contiene il client API generato e le utilità SSO per semplificare il lavoro con l'API.

- [Documentazione della libreria client API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### API pubbliche vs protette

Per il client API, ci sono tre moduli API, `api_default`, `api_public`, e `api_moderation`. `api_default` contiene i metodi che richiedono la tua API key, e `api_public` contiene chiamate API che possono essere effettuate direttamente da un browser/dispositivo mobile/etc senza autenticazione. Il modulo `api_moderation` contiene i metodi per la dashboard dei moderatori.

I metodi di `api_moderation` coprono elenchi, conteggi, ricerche ed esportazioni di commenti e dei loro log; azioni di moderazione come rimozione/restore dei commenti, segnalazione, impostazione dello stato di revisione/spam/approvazione, modifica dei voti e riapertura/chiusura dei thread; ban (bandire un utente da un commento, annullare un ban, riepiloghi pre-ban, stato e preferenze del ban e conteggi degli utenti bannati); e badge e fiducia (assegnazione/rimozione di un badge, elenco dei badge manuali, ottenere/impostare il fattore di fiducia di un utente e recuperare il profilo interno di un utente). Ogni metodo di `api_moderation` accetta un parametro `sso` in modo che la chiamata sia autenticata come moderatore SSO.