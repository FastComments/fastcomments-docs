### Installare le dipendenze

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Compilare dal sorgente

```bash
mkdir build
cd build
cmake ..
make
```

### Installare

```bash
sudo make install
```

### Contenuti della libreria

Questa libreria contiene il client API generato e le utility SSO per rendere più semplice il lavoro con l'API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### API pubbliche vs API protette

Per il client API, ci sono tre classi, `DefaultApi`, `PublicApi`, e `ModerationApi`. Il `DefaultApi` contiene metodi che richiedono la tua API key, e `PublicApi` contiene
metodi che possono essere effettuati direttamente da un browser/dispositivo mobile/etc senza autenticazione. Il `ModerationApi` contiene metodi che alimentano la dashboard del moderatore - elencare,
conteggio, ricerca, esportazione e recupero dei log per commenti, azioni di moderazione (rimuovere/ripristinare, segnalare, impostare stato di revisione/spam/approvazione, regolare i voti, riaprire/chiudere thread),
bannamenti (ban da un commento, annullare i ban, riepiloghi pre-ban, stato e preferenze del ban, conteggi utenti bannati), e badge & trust (assegnare/rimuovere badge, badge manuali, ottenere/impostare fattore di fiducia
, profilo interno utente). Ogni metodo di `ModerationApi` accetta un parametro `sso` in modo che la chiamata venga eseguita a nome di un moderatore autenticato via SSO.