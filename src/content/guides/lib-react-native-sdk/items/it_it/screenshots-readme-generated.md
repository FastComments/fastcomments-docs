Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Commenti in tempo reale</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Commenti in tempo reale, tema chiaro"/></td>
    <td align="center"><b>Tema scuro</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Commenti in tempo reale, tema scuro"/></td>
    <td align="center"><b>Chat dal vivo</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Preset di chat dal vivo"/></td>
  </tr>
</table>

### Editor di testo arricchito

Questa libreria utilizza [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) per l'editing di testo arricchito, che fornisce un'esperienza di editing WYSIWYG potente. Lo stesso editor alimenta iOS, Android e il web (tramite `react-native-web`), così il compositore si comporta in modo coerente su tutte le piattaforme con un'unica implementazione.

`react-native-enriched` richiede la New Architecture di React Native (Fabric) sulla parte nativa (predefinita a partire da RN 0.76, opzionale su RN 0.72‑0.75) e un bundler che risolva le condizioni di `exports` del pacchetto. Questo SDK è sviluppato e testato con RN 0.81 / React 19. Lo stesso editor gira anche sul web tramite `react-native-web`; la build web dell'editor enriched è ancora contrassegnata come sperimentale a monte.

### Widget

Il SDK fornisce tre widget, rispecchiando l'SDK Android di FastComments:

- `FastCommentsLiveCommenting` – commenti in thread con voti, risposte, paginazione, menzioni, notifiche e aggiornamenti in tempo reale.
- `FastCommentsLiveChat` – un preset di chat sullo stesso motore: messaggi cronologici con i nuovi in fondo, il compositore sotto l'elenco, una barra intestazione live (punto di connessione + conteggio utenti), cronologia infinita caricata scorrendo verso l'alto, scorrimento automatico verso i nuovi messaggi, nessun voto o thread di risposta. Ogni preset può essere sovrascritto tramite `config`.
- `FastCommentsFeed` – un feed sociale con compositore di post, media, reazioni, follow e banner live per nuovi post.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematizzazione

L'aspetto predefinito è generato da un set di token di design semantici (`FastCommentsTheme`): colori, spaziature, raggio, dimensioni dei caratteri, pesi dei caratteri e dimensioni degli avatar. Passa sovrascritture parziali dei token (tipizzati `FastCommentsThemeOverrides`) tramite la prop `theme` su qualsiasi widget e l'intero albero di stile verrà restilizzato in modo coerente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

La modalità scura è a un set di token di distanza:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` accetta ancora un albero grezzo `IFastCommentsStyles` per controlli chirurgici. Quando `theme` e `styles` sono entrambi forniti, gli stili espliciti prevalgono sull'albero tematico; quando è fornito solo `styles`, sostituisce totalmente i valori predefiniti (comportamento originale, quindi le integrazioni e i temi esistenti non ne sono influenzati). `setupDarkModeSkin` è deprecato a favore della prop `theme`.

### Opzioni di configurazione

Questa libreria mira a supportare tutte le opzioni di configurazione definite in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), proprio come l'implementazione web.

Oltre a queste, React Native aggiunge alcune opzioni specifiche dell'SDK tramite `FastCommentsRNConfig`:

- `hideTopBar` – nascondi la barra superiore dell'utente connesso / della campanella di notifica mostrata sopra il compositore.
- `usePressToEdit` – premi e tieni premuto un commento per aprire il suo menu.
- `disableDownVoting` – nascondi i pulsanti di voto negativo.
- `renderCommentInline` – visualizza le informazioni del commentatore all'interno dello stesso blocco HTML del contenuto del commento.
- `renderLikesToRight` – sposta l'area voto/like a destra del commento invece che sotto.
- `renderDateBelowComment` – visualizza la data sotto il commento.
- `showLiveStatus` – mostra la barra intestazione in stile chat "Live" + conteggio utenti sopra i commenti.
- `useInlineSubmitButton` – visualizza il pulsante invia come icona all'interno del compositore.
- `countAboveToggle` – con `useShowCommentsToggle`, quanti commenti vengono visualizzati sopra l'interruttore "Mostra commenti".
- `preserveFeedScrollPosition` – `FastCommentsFeed` ricorda il suo offset di scorrimento tra smontaggi/rimontaggi (predefinito vero).

### Concetti FastComments

I concetti principali da conoscere per iniziare sono `tenantId` e `urlId`. `tenantId` è l'identificatore del tuo account FastComments.com. `urlId` è a cosa verranno associati i thread di commenti. Può essere l'URL di una pagina, un ID prodotto, un ID articolo, ecc.

### Localizzazione

Tutto il testo rivolto all'utente in questi widget (etichette pulsanti, placeholder, stati vuoti, date relative come "5 minuti fa", messaggi di errore, ecc.) è **gestito dal server**. I componenti non codificano in modo fisso le stringhe inglesi; visualizzano le traduzioni che FastComments fornisce per la locale richiesta.

Per richiedere una locale, imposta `locale` nella tua configurazione:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, ecc.
};
```

Quando non è impostata alcuna `locale`, FastComments serve la lingua predefinita del tenant.

**Modifica del testo:** le traduzioni sono gestite nella tua dashboard FastComments, non in questo SDK. Per cambiare la formulazione, sovrascrivi il copy predefinito oppure aggiungi una lingua, modifica le traduzioni per il tuo account nella dashboard – il cambiamento viene automaticamente adottato dai widget senza necessità di rilasciare una nuova versione dell'app. L'SDK non fornisce fallback in inglese, quindi qualsiasi chiave lasciata vuota nella dashboard renderà un contenuto vuoto; mantieni le chiavi popolate per ogni locale supportato.

### Notifiche utente

FastComments supporta notifiche per [molti scenari](https://docs.fastcomments.com/guide-notifications.html). Le notifiche sono configurabili, possono essere disattivate globalmente o a livello di notifica/commento, e supportano le sottoscrizioni a livello di pagina così gli utenti possono iscriversi ai thread di una pagina o di un articolo specifici.

Ad esempio, è possibile utilizzare Secure SSO per autenticare l'utente e poi effettuare poll periodici per le notifiche non lette e spingerle all'utente.

Vedi [l'esempio AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) per capire come ottenere e tradurre le notifiche utente non lette.

### Browser GIF

Per impostazione predefinita, nessuna selezione di immagini o GIF è abilitata. Consulta [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) per sapere come supportare caricamenti di immagini e GIF. È presente un Browser GIF che anonimizza le ricerche e le immagini fornite in questa libreria; devi semplicemente usarlo.

### Prestazioni

Apri un ticket con un esempio da riprodurre, includendo il dispositivo usato, se identifichi problemi di prestazioni. Le prestazioni sono una priorità fondamentale in tutte le librerie FastComments.