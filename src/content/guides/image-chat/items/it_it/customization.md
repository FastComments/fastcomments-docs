### Supporto per la Modalità Scura

Image Chat include il supporto integrato per la modalità scura. Quando imposti `hasDarkBackground: true` nella tua configurazione, le finestre della chat e gli elementi dell'interfaccia si adattano automaticamente per funzionare bene su sfondi scuri.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Lo stile per la modalità scura si applica alle finestre della chat, ai quadrati dei marker e a tutti gli elementi interattivi. Se il tuo sito dispone di un interruttore per la modalità scura, puoi reinizializzare il widget quando la modalità cambia, oppure usare l'approccio della classe sul body descritto più avanti.

### Modalità Scura Dinamica

Se la modalità scura del tuo sito è controllata aggiungendo la classe `.dark` all'elemento body, l'interfaccia di Image Chat la rispetterà automaticamente senza necessità di reinizializzazione. Gli stili del widget sono progettati per rispondere alla presenza di questa classe.

```css
/* Il tuo CSS per la modalità scura */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Personalizzazione con CSS

Puoi personalizzare l'aspetto dei marker, delle finestre della chat e di altri elementi utilizzando il CSS. Il widget aggiunge classi specifiche che puoi prendere di mira nel tuo foglio di stile.

I quadrati e le finestre della chat utilizzano il sistema di stile delle bolle di commento di FastComments, quindi qualsiasi personalizzazione che hai applicato al widget di commenti standard influenzerà anche Image Chat.

### Dimensione dei Quadrati della Chat

L'opzione `chatSquarePercentage` controlla la dimensione dei marker cliccabili. Il valore predefinito è il 5% della larghezza dell'immagine:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Quadrati più grandi e più visibili
});
```

Valori più piccoli creano marker più discreti che si fondono con l'immagine. Valori più grandi rendono i marker più evidenti e più facili da cliccare, specialmente sui dispositivi mobili o per motivi di accessibilità.

### Comportamento sui dispositivi mobili

Su schermi con larghezza inferiore a 768px, Image Chat passa automaticamente a un layout ottimizzato per dispositivi mobili. Le finestre della chat appaiono a schermo intero invece di fluttuare accanto ai marker, offrendo una migliore usabilità su schermi piccoli.

I marker rimangono visibili nelle loro posizioni responsive sull'immagine. Gli utenti possono toccare un qualsiasi marker per aprire l'interfaccia della chat a schermo intero. Questo comportamento è integrato e non richiede configurazione.

### Aspetto delle Finestre della Chat

Le finestre della chat sono larghe 300px su desktop con una freccia di 16px che punta al marker. Le finestre si posizionano automaticamente in base allo spazio disponibile nella viewport, utilizzando classi di posizionamento come `to-right`, `to-left`, `to-top` e `to-bottom`.

Puoi aggiungere CSS personalizzato per regolare colori, font, spaziatura o altre proprietà visive di queste finestre. Le finestre della chat usano la stessa struttura di componenti del widget standard di FastComments, quindi ereditano qualsiasi personalizzazione globale che hai applicato.

### Inizializzazione ritardata

Le finestre della chat si inizializzano al passaggio del mouse per gli utenti desktop o immediatamente quando vengono create. Questo riduce il carico iniziale renderizzando l'interfaccia della chat solo quando gli utenti interagiscono effettivamente con un marker.

L'inizializzazione ritardata avviene in modo trasparente. Gli utenti non notano alcun ritardo, ma il browser non deve renderizzare dozzine di finestre della chat nascoste se hai molti marker su un'immagine.

### Localizzazione

Image Chat supporta tutte le stesse opzioni di localizzazione del widget standard di FastComments. Imposta l'opzione `locale` per visualizzare il testo dell'interfaccia in lingue diverse:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Francese
});
```

FastComments supporta decine di lingue. L'impostazione della locale influisce su tutto il testo dell'interfaccia, inclusi prompt, pulsanti e testo segnaposto.

### Opzioni di Personalizzazione Ereditate

Poiché Image Chat estende il widget di commenti standard, eredita tutte le opzioni di personalizzazione del widget base. Questo include classi CSS personalizzate, traduzioni personalizzate, personalizzazione degli avatar, formattazione delle date e molto altro.

Consulta la documentazione principale di personalizzazione di FastComments per l'elenco completo delle opzioni di personalizzazione disponibili.

### Uso di Font Personalizzati

Se il tuo sito usa font personalizzati, l'interfaccia di Image Chat erediterà quei font dal CSS della tua pagina. Le finestre della chat vengono renderizzate all'interno del DOM della tua pagina e rispettano le impostazioni tipografiche esistenti.

Per risultati ottimali, assicurati che i tuoi font personalizzati siano caricati prima di inizializzare Image Chat, oppure accetta che possa esserci un breve flash di testo non stilizzato mentre i font vengono caricati.

### Design Visivo dei Marker

I marker quadrati hanno un design visivo sottile che li rende visibili senza sovrastare l'immagine. Puoi personalizzare il loro aspetto con il CSS se desideri un trattamento visivo diverso.

I marker includono stati al passaggio del mouse che forniscono un feedback quando gli utenti ci passano sopra con il mouse. Su dispositivi touch, l'interazione tap fornisce un feedback immediato aprendo la finestra della chat.

---