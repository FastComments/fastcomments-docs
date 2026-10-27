---
Per impostazione predefinita, FastComments traduce l'interfaccia del widget (pulsanti, etichette, messaggi di errore) nella lingua del visualizzatore, ma i commenti vengono mostrati esattamente come sono stati scritti.

La Traduzione dei Commenti aggiunge la traduzione dei commenti stessi, così una comunità che scrive in più di una lingua può leggere ogni commento nella propria lingua.

Ci sono due modalità:

- **Automatico**: i commenti scritti in una lingua diversa da quella del widget vengono mostrati tradotti. Una riga "Tradotto dallo spagnolo" appare sotto il commento con un link **Mostra originale** che alterna tra la traduzione e il testo non tradotto.
- **Manuale**: ogni commento ottiene un link **Traduci**. Cliccandolo si traduce quel singolo commento nella lingua del visualizzatore, con lo stesso interruttore **Mostra originale** successivamente. I commenti che sono già nella lingua del visualizzatore lo indicano invece.

La lingua di destinazione è la lingua del widget: l'impostazione [Locale / Language](#locale) sulla stessa regola di personalizzazione, se impostata, altrimenti la lingua del browser del visitatore. Non è richiesto nulla al visitatore, né account, né estensione o software aggiuntivo, e funziona allo stesso modo nel widget incorporato e nelle pagine dei commenti autonome.

Questo è attivato per <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regola di personalizzazione</a>, quindi può essere applicato all'intero sito, a un dominio o a una singola pagina. È disattivato per impostazione predefinita.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Menu a discesa Traduzione dei Commenti nella pagina di personalizzazione del widget con le opzioni Off, Automatico e Manuale'; title='Traduzione dei Commenti' app-screenshot-end]

La traduzione è un'impostazione lato server. Passare `commentTranslation` nella configurazione JavaScript del widget non la abilita; la regola lo fa.

Ogni commento viene tradotto al massimo una volta per lingua di destinazione e il risultato viene memorizzato con il commento, così un commento letto da mille parlanti spagnolo viene tradotto una sola volta. Se l'autore o un moderatore modifica il commento, la traduzione memorizzata viene scartata e il commento viene tradotto nuovamente alla visualizzazione successiva. Il testo tradotto passa attraverso le stesse regole di sanitizzazione e di collegamento del commento originale. I commenti troppo lunghi, o la cui lingua non può essere rilevata, vengono mostrati non tradotti.

L'utilizzo viene fatturato come token LLM allo stesso tasso di [AI Agents](/guide-ai-agents.html#plans-and-eligibility) e appare nello stesso punto nei dettagli di fatturazione.

---