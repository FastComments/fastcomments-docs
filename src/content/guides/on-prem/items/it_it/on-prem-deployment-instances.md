### Componenti necessari

Per On-Prem, FastComments consiste solo in un server applicativo e un database. Abbiamo semplificato il deployment in modo che
l'applicazione possa servire tutto il traffico direttamente senza aggiungere altri componenti.

Il server applicativo è fornito in un'immagine Docker e può essere distribuito con qualsiasi soluzione di gestione dei container.

Il database, MongoDB, può essere gestito autonomamente o ospitato da un altro provider come AWS DocumentDB o MongoDB Atlas.

FastComments è attualmente testato con MongoDB 7, tuttavia puntiamo a essere compatibili con DocumentDB per facilitare il deployment.

### Dimensioni delle istanze

Scoprirai che FastComments è abbastanza ottimizzato e non richiede macchine grandi per l'applicazione stessa per mantenere P99 bassi.

Tutti i job batch e cron utilizzano lo streaming per limitare l'uso totale della memoria.

Le tabelle sottostanti per il server applicativo e il database possono aiutare con il dimensionamento.

### Istanze del server applicativo


| Utenti concorrenti | CPU totali del cluster | Memoria totale del cluster |
|--------------------|------------------------|----------------------------|
| 100                | 1                      | 256mb                      |
| 1K                 | 2                      | 512mb                      |
| 10K                | 8                      | 1gb                        |
| 100K               | 32                     | 8gb                        |
| 1M                 | 64                     | 64gb                       |

Ad esempio, un singolo core che serve circa 100 thread di commenti al secondo di solito non usa mai più di 250mb di RSS.

### Istanze del server database

Il dimensionamento del database dipende dalla dimensione del working set, che è la quantità di dati a cui si accede in un dato momento, oltre che dalle richieste concorrenti.

FastComments è abbastanza gentile con Mongo, in quanto per le query "hot" usa index hints, cursori in streaming e ha limiti di concorrenza in varie aree
per evitare il sovraccarico dei sistemi downstream.

Di seguito è riportata una linea guida generale sulle dimensioni delle istanze del database. **Nota che questo è __per istanza__, non le risorse totali nel cluster**.

| Utenti concorrenti | Commenti memorizzati | CPU per istanza | Memoria per istanza |
|--------------------|----------------------|-----------------|---------------------|
| 100                | 1k                   | 1               | 256mb               |
| 1K                 | 5k                   | 2               | 512mb               |
| 10K                | 100k                 | 8               | 2gb                 |
| 100K               | 500k                 | 16              | 8gb                 |
| 1M                 | 5M                   | 32              | 32gb                |

Le tabelle sopra sono stime conservative. Potresti scoprire che i requisiti effettivi differiscono in base alla tua configurazione specifica (dimensione delle pagine, volume dei commenti, ecc).