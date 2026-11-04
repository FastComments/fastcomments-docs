Commentazione live in thread con avatar, risposte annidate, voti e il composer rich-text integrato, più un tema scuro e un preset di live chat (mostrato qui renderizzato tramite `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Commentazione Live</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Commentazione live, tema chiaro"/></td>
    <td align="center"><b>Tema Scuro</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Commentazione live, tema scuro"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset live chat"/></td>
  </tr>
</table>

### Editor Rich Text

Questa libreria utilizza [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) per la modifica di testo formattato, che offre una potente esperienza di editing WYSIWYG. Lo stesso editor alimenta iOS, Android e il web (tramite `react-native-web`), quindi il composer si comporta in modo coerente su ogni piattaforma con un'unica implementazione.

`react-native-enriched` richiede la React Native New Architecture (Fabric) su nativo (predefinita da RN 0.76, opzionale su RN 0.72-0.75) e un bundler che risolva le condizioni `exports` dei pacchetti. Questo SDK è sviluppato e testato con RN 0.81 / React 19. Lo stesso editor funziona anche sul web tramite `react-native-web`; la build web dell'editor enriched è ancora contrassegnata come sperimentale a monte.

### Widget

L'SDK include tre widget, in linea con l'SDK Android di FastComments:

- `FastCommentsLiveCommenting` - commentazione in thread con voti, risposte, paginazione, menzioni, notifiche e aggiornamenti live.
- `FastCommentsLiveChat` - un preset di chat sullo stesso motore: messaggi cronologici con i nuovi in basso, il composer sotto l'elenco, una barra header live (punto di connessione + conteggio utenti), cronologia infinita caricata scorrendo verso l'alto, auto-scroll verso i nuovi messaggi, senza voti né thread di risposte. Ogni preset può essere sovrascritto tramite `config`.
- `FastCommentsFeed` - un feed social con composer per i post, media, reazioni, follow e banner live per i nuovi post.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

L'aspetto predefinito è generato da un insieme di design token semantici (`FastCommentsTheme`): colori, spaziature, raggi, dimensioni dei font, pesi dei font e dimensioni degli avatar. Passa override parziali dei token (tipizzati `FastCommentsThemeOverrides`) tramite la prop `theme` su qualsiasi widget e l'intero albero di stili viene ridisegnato in modo coerente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

La modalità scura è a un set di token di distanza:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

La prop `styles` accetta ancora un albero `IFastCommentsStyles` grezzo per un controllo chirurgico. Quando `theme` e `styles` sono forniti entrambi, gli stili espliciti prevalgono sull'albero a tema; quando viene fornito solo `styles`, sostituisce interamente i valori predefiniti (il comportamento originale, quindi le integrazioni e le skin esistenti non sono influenzate). `setupDarkModeSkin` è deprecato in favore della prop `theme`.

### Opzioni di Configurazione

Questa libreria mira a supportare tutte le opzioni di configurazione definite in [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), proprio come l'implementazione web.

Oltre a queste, React Native aggiunge alcune opzioni specifiche dell'SDK tramite `FastCommentsRNConfig`:

- `hideTopBar` - nasconde la barra utente connesso / campanella notifiche mostrata sopra il composer.
- `usePressToEdit` - tieni premuto un commento per aprire il suo menu.
- `disableDownVoting` - nasconde i pulsanti di voto negativo.
- `renderCommentInline` - renderizza le informazioni del commentatore nello stesso blocco HTML del contenuto del commento.
- `renderLikesToRight` - sposta l'area voto/mi piace a destra del commento invece che sotto.
- `renderDateBelowComment` - renderizza la data sotto il commento.
- `showLiveStatus` - mostra la barra header in stile chat "Live" + conteggio utenti sopra i commenti.
- `useInlineSubmitButton` - renderizza il pulsante di invio come icona all'interno del composer.
- `countAboveToggle` - con `useShowCommentsToggle`, quanti commenti renderizzare sopra l'interruttore "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` ricorda la propria posizione di scroll tra unmount/remount (predefinito true).

### Concetti di FastComments

I concetti principali da conoscere per iniziare sono `tenantId` e `urlId`. `tenantId` è l'identificatore del tuo account FastComments.com. `urlId` è a cosa saranno legati i thread di commenti. Può essere l'URL di una pagina, o un id di prodotto, un id di articolo, ecc.

### Localizzazione

Tutti i testi rivolti all'utente in questi widget (etichette dei pulsanti, placeholder, stati vuoti, date relative come "5 minuti fa", messaggi di errore, ecc.) sono **gestiti dal server**. I componenti non codificano stringhe in inglese; renderizzano le traduzioni che FastComments fornisce per la locale richiesta.

Per richiedere una locale, imposta `locale` nella tua config:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, ecc.
};
```

Quando non è impostata alcuna `locale`, FastComments fornisce la lingua predefinita del tenant.

**Modifica del testo:** le traduzioni sono gestite nella tua dashboard FastComments, non in questo SDK. Per cambiare la formulazione, sovrascrivere il testo predefinito o aggiungere una lingua, modifica le traduzioni del tuo account nella dashboard - la modifica viene recepita dai widget automaticamente senza richiedere il rilascio dell'app. L'SDK non include fallback in inglese, quindi qualsiasi chiave che svuoti nella dashboard verrà renderizzata vuota; mantieni le chiavi compilate per ogni locale che supporti.

### Notifiche Utente

FastComments supporta le notifiche per [molti scenari](https://docs.fastcomments.com/guide-notifications.html). Le notifiche sono configurabili, possono essere disattivate globalmente o a livello di notifica/commento, e supportano sottoscrizioni a livello di pagina in modo che gli utenti possano iscriversi ai thread di una pagina o articolo specifico.

Ad esempio, è possibile usare Secure SSO per autenticare l'utente e poi interrogare periodicamente le notifiche non lette e inviarle all'utente.

Vedi [l'esempio AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) per come ottenere e tradurre le notifiche utente non lette.

### Browser Gif

Per impostazione predefinita, nessuna selezione di immagini o gif è abilitata. Vedi [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) per come supportare il caricamento di immagini e gif. C'è un Gif Browser che anonimizza le ricerche e le immagini fornito in questa libreria, ti basta usarlo.

### Prestazioni

Apri un ticket con un esempio per riprodurre il problema, includendo il dispositivo utilizzato, se identifichi problemi di prestazioni. Le prestazioni sono un cittadino di prima classe in tutte le librerie FastComments.