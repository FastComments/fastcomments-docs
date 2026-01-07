### Esempio di formato

Il CSV dei Commenti Programmati appare cosi:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Dettagli del formato

Il file CSV dei Commenti Programmati supporta le seguenti colonne:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

Le seguenti colonne sono **obbligatorie**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Avrai bisogno della colonna Parent ID se vuoi supportare risposte nidificate automatizzate.

### Come funziona il formato

Il formato di importazione funziona cosi:

1. Definisci una riga nel CSV per ogni commento che vuoi pubblicare.
2. Il commento viene pubblicato dopo il ritardo specificato (ore + minuti + secondi).
   1. Per le importazioni programmate manualmente, i ritardi sono relativi a quando premi "play" nell'interfaccia, dopo che l'importazione e completata - **non quando l'importazione inizia**.
   2. Per le importazioni programmate automaticamente, il ritardo e dal momento del caricamento della pagina.
3. Devi definire un ID. Puoi semplicemente usare identificatori incrementali come 1, 2, 3, 4, 5.
4. Se vuoi usare risposte nidificate, imposta semplicemente il valore della colonna `Parent ID` sull'`ID` di un altro commento.
5. Il campo `Comment` supporta la stessa sintassi che FastComments supporta nel suo widget commenti per stilizzare il testo e aggiungere immagini.
6. Il campo `Avatar`, se usato, deve essere un'immagine accessibile pubblicamente. Sara copiata e servita dal nostro CDN.
7. Puoi eliminare tutti i commenti dopo la riproduzione, o se la riproduzione viene fermata.
8. L'eliminazione avviene in tempo reale, quindi puoi riutilizzare la stessa importazione programmata piu e piu volte.

### Un esempio

[Un file CSV di esempio e qui](/csv/fastcomments-scheduled-comments-example.csv).
