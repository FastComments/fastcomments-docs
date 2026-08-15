Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Commenti Live</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Commenti live, tema chiaro"/></td>
    <td align="center"><b>Tema Scuro</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Commenti live, tema scuro"/></td>
    <td align="center"><b>Chat Live</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset di live chat"/></td>
  </tr>
</table>

### Editor di Testo Formattato

Questa libreria utilizza [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) per l'editing di testo formattato, che offre un'esperienza di editing WYSIWYG potente. Lo stesso editor alimenta iOS, Android e il web (tramite `react-native-web`), quindi il compositore si comporta in modo coerente su ogni piattaforma con un'unica implementazione.

`react-native-enriched` richiede la New Architecture di React Native (Fabric) su native (impostazione predefinita da RN 0.76, opzionale su RN 0.72-0.75) e un bundler che risolve le condizioni `exports` del pacchetto. Questo SDK è sviluppato e testato con RN 0.81 / React 19. Lo stesso editor funziona anche sul web tramite `react-native-web`; la build web dell'editor enriched è ancora contrassegnata come sperimentale a monte.

### Widget

L'SDK fornisce tre widget, rispecchiando l'SDK Android di FastComments:

- `FastCommentsLiveCommenting` - commenti in thread con voti, risposte, paginazione, menzioni, notifiche e aggiornamenti live.
- `FastCommentsLiveChat` - un preset di chat basato sullo stesso motore: messaggi cronologici con i nuovi in fondo, il compositore sotto la lista, una barra intestazione live (punto di connessione + conteggio utenti), cronologia infinita caricata scorrendo verso l'alto, auto-scroll ai nuovi messaggi, senza voti o thread di risposta. Ogni preset può essere sovrascritto tramite `config`.
- `FastCommentsFeed` - un feed sociale con compositore di post, media, reazioni, follow e banner live per nuovi post.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematizzazione

L'aspetto predefinito è generato da un insieme di token di design semantici (`FastCommentsTheme`): colori, spaziatura, raggio, dimensioni dei font, pesi dei font e dimensioni degli avatar. Passa sovrascritture parziali dei token (tipizzate `FastCommentsThemeOverrides`) tramite la prop `theme` su qualsiasi widget e l'intero albero di stile verrà ri-stilizzato in modo coerente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

La modalità scura è a un set di token di distanza:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` accetta ancora un albero grezzo `IFastCommentsStyles` per un controllo chirurgico. Quando `theme` e `styles` sono entrambi forniti, gli stili espliciti prevalgono sull'albero tematico; quando è fornito solo `styles`, sostituisce completamente i valori predefiniti (il comportamento originale, quindi le integrazioni e i temi esistenti non sono influenzati). `setupDarkModeSkin` è deprecato a favore della prop `theme`.

### Opzioni di Configurazione

Questa libreria mira a supportare tutte le opzioni di configurazione definite in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), proprio come l'implementazione web.

Oltre a queste, React Native aggiunge alcune opzioni specifiche dell'SDK tramite `FastCommentsRNConfig`:

- `hideTopBar` - nasconde la barra superiore dell'utente loggato / campanella di notifica mostrata sopra il compositore.
- `usePressToEdit` - premi e tieni premuto un commento per aprire il suo menu.
- `disableDownVoting` - nasconde i pulsanti di voto negativo.
- `renderCommentInline` - rende le informazioni del commentatore all'interno dello stesso blocco HTML del contenuto del commento.
- `renderLikesToRight` - sposta l'area voto/like a destra del commento invece che sotto.
- `renderDateBelowComment` - rende la data sotto il commento.
- `showLiveStatus` - mostra la barra intestazione in stile chat "Live" + conteggio utenti sopra i commenti.
- `useInlineSubmitButton` - rende il pulsante di invio come icona all'interno del compositore.
- `countAboveToggle` - con `useShowCommentsToggle`, quanti commenti vengono renderizzati sopra il toggle "Mostra Commenti".
- `preserveFeedScrollPosition` - `FastCommentsFeed` ricorda il suo offset di scorrimento tra smontaggio/rimontaggio (predefinito true).

### Concetti di FastComments

I concetti principali da conoscere per iniziare sono `tenantId` e `urlId`. `tenantId` è l'identificatore del tuo account FastComments.com. `urlId` è a cosa saranno associati i thread di commenti. Può essere l'URL di una pagina, un ID prodotto, un ID articolo, ecc.

### Localizzazione

Tutto il testo rivolto all'utente in questi widget (etichette dei pulsanti, segnaposti, stati vuoti, date relative come "5 minuti fa", messaggi di errore, ecc.) è **guidato dal server**. I componenti non codificano in modo fisso le stringhe inglesi; renderizzano le traduzioni che FastComments fornisce per la lingua richiesta.

Per richiedere una lingua, imposta `locale` nella tua configurazione:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Quando `locale` non è impostato, FastComments fornisce la lingua predefinita del tenant.

**Modifica del testo:** le traduzioni sono gestite nella tua dashboard FastComments, non in questo SDK. Per cambiare la formulazione, sovrascrivi il testo predefinito o aggiungi una lingua, modifica le traduzioni per il tuo account nella dashboard – la modifica viene rilevata automaticamente dai widget senza necessità di rilasciare l'app. L'SDK non fornisce fallback in inglese, quindi qualsiasi chiave che lasci vuota nella dashboard verrà renderizzata vuota; mantieni le chiavi popolate per ogni lingua supportata.

### Notifiche Utente

FastComments supporta le notifiche per [molti scenari](https://docs.fastcomments.com/guide-notifications.html). Le notifiche sono configurabili, possono essere disattivate globalmente o a livello di notifica/commento, e supportano le sottoscrizioni a livello di pagina così gli utenti possono iscriversi ai thread di una pagina o articolo specifico.

Ad esempio, è possibile utilizzare Secure SSO per autenticare l'utente e poi effettuare polling periodico per le notifiche non lette e inviarle all'utente.

Vedi [l'esempio AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) per capire come ottenere e tradurre le notifiche non lette dell'utente.

### Browser di Gif

Per impostazione predefinita, non è abilitata alcuna selezione di immagini o gif. Vedi [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) per sapere come supportare il caricamento di immagini e gif. C'è un Browser di Gif che anonimizza le ricerche e le immagini fornite in questa libreria, devi semplicemente usarlo.

### Prestazioni

Apri un ticket con un esempio da riprodurre, includendo il dispositivo usato, se identifichi problemi di prestazioni. Le prestazioni sono una priorità assoluta in tutte le librerie FastComments.