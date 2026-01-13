Per impostazione predefinita, FastComments rileverà automaticamente se il tuo sito ha uno sfondo scuro basandosi sulla "distanza dal nero" nel cerchio dei colori.

I nostri prodotti fanno del loro meglio in questo, tuttavia ci sono quasi infiniti colori nella ruota dei colori, e potrebbero esserci scenari in cui l'applicazione sceglie di usare la modalità scura quando non è appropriata, e viceversa. Questa documentazione spiega come avere un controllo più preciso su questo.

#### Dettagli tecnici

Rileviamo la modalità scura attraversando gli elementi nella pagina verso l'alto dal widget dei commenti, cercando uno sfondo scuro quando il widget viene caricato inizialmente.

Per attivare/disattivare la modalità scura dopo questo passaggio, devi chiamare il widget per aggiornare la sua configurazione. Questo è trattato nella sezione `Configurazione manuale`.
