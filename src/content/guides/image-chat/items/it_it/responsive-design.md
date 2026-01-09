### Posizionamento Basato su Percentuali

Image Chat usa coordinate basate su percentuali invece di coordinate in pixel per posizionare i marker di chat sulle immagini. Quando un utente clicca su un'immagine, il widget converte le coordinate in pixel del clic in percentuali della larghezza e dell'altezza dell'immagine. Questo approccio garantisce che i marker rimangano nella posizione corretta indipendentemente da come l'immagine viene visualizzata.

Ad esempio, se un utente clicca a 250 pixel dal bordo sinistro di un'immagine larga 1000px, il widget memorizza questo come 25% dal bordo sinistro. Quando un altro utente visualizza la stessa immagine a 500px di larghezza su un dispositivo mobile, il marker appare a 125 pixel dal bordo sinistro, che è ancora il 25% della larghezza.

### Vantaggi per layout responsive

Questo sistema basato su percentuali fa sì che Image Chat funzioni senza problemi su tutte le dimensioni e orientamenti dei dispositivi. Le tue immagini potrebbero essere visualizzate a dimensioni diverse a seconda della larghezza dello schermo, delle regole CSS o dei vincoli del contenitore, ma i marker si allineano sempre correttamente con le posizioni previste.

Gli utenti su computer desktop con monitor grandi vedono i marker nelle stesse posizioni relative degli utenti su smartphone con schermi piccoli. I marker si ridimensionano proporzionalmente all'immagine stessa.

### Ridimensionamento dell'Immagine e Rapporto d'Aspetto

Finché la tua immagine mantiene il rapporto d'aspetto durante il ridimensionamento (che è il comportamento predefinito del browser), i marker basati su percentuali rimarranno perfettamente allineati. Il widget presuppone che le immagini si ridimensionino proporzionalmente e calcola le posizioni basandosi su questa assunzione.

Se applichi CSS che distorce il rapporto d'aspetto dell'immagine (ad esempio usando `object-fit: cover` con dimensioni specifiche), le posizioni dei marker potrebbero non allinearsi correttamente. Per risultati ottimali, lascia che le immagini si ridimensionino naturalmente oppure usa `object-fit: contain` per mantenere il rapporto d'aspetto.

### Dimensionamento del quadrato di chat

La dimensione visiva dei marker di chat è anch'essa basata su percentuali. L'opzione di configurazione `chatSquarePercentage` ha valore predefinito 5%, il che significa che ogni quadrato corrisponde al 5% della larghezza dell'immagine. Questo garantisce un peso visivo coerente tra immagini di diverse dimensioni.

Su un'immagine larga 1000px con l'impostazione predefinita del 5%, i marker sono quadrati da 50px. Su un'immagine larga 500px, gli stessi marker sono quadrati da 25px. Rimangono proporzionali alla dimensione dell'immagine.

### Comportamento su Mobile

Su schermi con larghezza inferiore a 768px, Image Chat passa a un layout ottimizzato per mobile. Le finestre di chat si aprono a schermo intero invece di fluttuare accanto al marker. Questo offre una migliore usabilità su schermi piccoli dove le finestre flottanti oscurerebbero troppo l'immagine.

I marker stessi restano visibili e cliccabili alle loro posizioni basate su percentuali. Gli utenti possono comunque vedere dove sono localizzate le discussioni e toccare i marker per aprire l'interfaccia di chat a schermo intero.

### Caricamento Dinamico delle Immagini

Il sistema basato su percentuali funziona correttamente anche quando le immagini vengono caricate in modo asincrono o cambiano dimensione dopo il caricamento della pagina. Il widget monitora l'elemento immagine e ricalcola le posizioni dei marker ogni volta che cambiano le dimensioni dell'immagine.

Se stai usando il lazy-loading delle immagini o implementando immagini responsive con dimensioni diverse a differenti breakpoint, i marker si adattano automaticamente quando cambia la dimensione dell'immagine.

### Coerenza tra Dispositivi

Poiché le coordinate vengono memorizzate come percentuali nel database, una discussione creata su un computer desktop appare esattamente nella stessa posizione relativa quando viene visualizzata su un tablet o uno smartphone. Gli utenti possono collaborare tra dispositivi senza incongruenze nel posizionamento.

Questo funziona in entrambe le direzioni. Una discussione creata toccando un punto specifico su un dispositivo mobile appare nella stessa posizione relativa quando viene visualizzata su un grande monitor desktop.

### Considerazioni sul Viewport

Il widget calcola le percentuali rispetto all'elemento immagine stesso, non rispetto al viewport. Ciò significa che lo scorrimento della pagina o la modifica delle dimensioni della finestra del browser non influiscono sulle posizioni dei marker. I marker restano ancorati alle loro posizioni sull'immagine indipendentemente dai cambiamenti del viewport.

### Protezione futura dei contenuti

L'approccio basato su percentuali rende le tue discussioni sulle immagini resilienti ai cambiamenti di layout, design o ecosistema di dispositivi. Con l'emergere di nuove dimensioni di schermo e dispositivi, le discussioni esistenti continueranno a essere visualizzate correttamente senza richiedere aggiornamenti o migrazioni.