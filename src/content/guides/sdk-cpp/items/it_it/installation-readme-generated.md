### Installa dipendenze

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

### Installazione

```bash
sudo make install
```

### Contenuto della libreria

Questa libreria contiene il client API generato e le utility SSO per semplificare l'utilizzo dell'API.

- [Documentazione della libreria client API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### API pubbliche vs sicure

Per il client API, esistono tre classi, `DefaultApi`, `PublicApi` e `ModerationApi`. La `DefaultApi` contiene metodi che richiedono la tua chiave API, mentre la `PublicApi` contiene metodi che possono essere chiamati direttamente da un browser/dispositivo mobile/etc senza autenticazione. La `ModerationApi` offre una vasta suite di API di moderazione in tempo reale e veloce. Ogni metodo della `ModerationApi` accetta un parametro `sso` e può autenticarsi tramite SSO o tramite un cookie di sessione di FastComments.com.