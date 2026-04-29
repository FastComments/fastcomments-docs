Per impostazione predefinita un agente viene eseguito **immediatamente** dopo che il suo trigger si attiva. Il campo **Delay before running** nel modulo di modifica modifica questo comportamento: la piattaforma mette in coda il trigger ed esegue l'agente all'orario programmato.

### Quando usare un ritardo

- **Trigger a soglia di segnalazioni** - le segnalazioni spesso arrivano a raffica. Un ritardo di 10-30 minuti permette alla situazione di stabilizzarsi così che l'agente agisca sul conteggio finale delle segnalazioni invece che sul momento dell'arrivo.
- **Trigger a soglia di voti** - stessa logica, particolarmente per campagne coordinate di downvote.
- **Riassunto del thread** - il [Thread Summarizer template](#template-thread-summarizer) usa per impostazione predefinita un ritardo di 30 minuti in modo da riassumere una conversazione che ha avuto il tempo di svilupparsi, non un thread con due risposte.
- **Periodo di raffreddamento / rivalutazione** - "24 ore dopo che un commento è stato bloccato, considerare se sbloccarlo."

### Configurazione

- **Campo**: Delay before running.
- **Intervallo**: da 0 a 2.592.000 secondi (30 giorni).
- **Unità**: secondi, minuti, ore o giorni.

### Idempotenza

La coda differita non elimina i duplicati dei trigger. Due segnalazioni che arrivano a 1 secondo di distanza su un agente con ritardo di 30 minuti programmeranno entrambe un'esecuzione 30 minuti dopo, e l'agente verrà eseguito **due volte**, entrambe le volte sullo stesso contesto (per lo più). Se vuoi una semantica "al massimo una esecuzione per finestra", l'agente deve farla rispettare — tipicamente scrivendo una [nota di memoria](#tools-overview) alla prima esecuzione e verificandola nelle esecuzioni successive.

### Nota sui costi

I trigger differiti vengono registrati **prima** della loro esecuzione. Un picco di trigger su un agente con ritardo elevato può accumularsi nella coda senza consumare token; il costo viene pagato solo quando il cron li dispatcha. Usa [Run History](#run-history) e [Drop Reasons](#drop-reasons) per vedere quanto spesso i trigger differiti vengono effettivamente eseguiti rispetto a quante volte vengono scartati a runtime per motivi di budget.

### Il replay non rispetta il ritardo

La funzionalità [Test Runs (Replays)](#test-runs-replays) esegue l'agente immediatamente contro commenti storici — non attende il ritardo configurato. Consideralo una caratteristica: i replay servono a visualizzare in anteprima ciò che l'agente **farebbe** dato il contesto, non a riprodurre la pianificazione in tempo reale.