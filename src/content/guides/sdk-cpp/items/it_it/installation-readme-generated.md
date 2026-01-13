### Installare le dipendenze

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Compilazione dal sorgente

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

Questa libreria contiene il client API generato e le utility SSO per rendere più semplice il lavoro con l'API.

- [Documentazione della libreria client API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### API pubbliche vs protette

Per il client API, ci sono due classi, `DefaultAPI` e `PublicAPI`. La `DefaultAPI` contiene metodi che richiedono la tua chiave API, mentre la `PublicAPI` contiene chiamate API che possono essere effettuate direttamente da un browser/dispositivo mobile/etc. senza autenticazione.