### Panoramica

FastComments Image Chat estende il widget di commenti standard di FastComments, quindi eredita tutte le opzioni di configurazione dal widget base aggiungendo alcune opzioni specifiche per le annotazioni sulle immagini.

### Configurazione richiesta

#### tenantId

Il tuo FastComments Tenant ID è obbligatorio. Puoi trovarlo nella tua [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Opzioni specifiche di Image Chat

#### urlId

Per impostazione predefinita, Image Chat genera un identificatore univoco per ogni conversazione basato sull'URL della pagina, sulla sorgente dell'immagine e sulle coordinate X/Y. Puoi sovrascriverlo con un `urlId` personalizzato.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Questo è utile quando la struttura degli URL potrebbe cambiare ma vuoi mantenere le stesse conversazioni, o quando vuoi condividere le annotazioni su più pagine.

#### chatSquarePercentage

Controlla la dimensione dei marker cliccabili come percentuale della larghezza dell'immagine. Il valore predefinito è 5%, il che significa che ogni marker è il 5% della larghezza dell'immagine.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Più grandi, marcatori più visibili
});
```

Valori più piccoli creano marker meno invasivi, più adatti per immagini dettagliate. Valori più grandi rendono i marker più facili da vedere e cliccare su immagini affollate o per utenti su dispositivi mobili.

#### hasDarkBackground

Abilita lo stile in modalità scura quando la tua pagina ha uno sfondo scuro.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Una funzione di callback che viene eseguita ogni volta che il conteggio dei commenti cambia. Questo è utile per aggiornare elementi dell'interfaccia come badge o titoli della pagina.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Opzioni di configurazione ereditate

Poiché Image Chat estende il widget di commenti standard, puoi usare qualsiasi opzione di configurazione del widget base di FastComments. Ecco alcune opzioni comunemente usate:

#### locale

Imposta la lingua per l'interfaccia del widget. FastComments supporta decine di lingue.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spagnolo
});
```

#### readonly

Rendi tutte le conversazioni in sola lettura. Gli utenti possono visualizzare i marker e le discussioni esistenti ma non possono crearne di nuovi o rispondere.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso e simpleSSO

Integra il tuo sistema di autenticazione utilizzando il Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Configurazione SSO
    }
});
```

Vedi la documentazione SSO per i dettagli completi sulle opzioni di autenticazione.

#### maxReplyDepth

Controlla quanti livelli di profondità possono avere le risposte. Per impostazione predefinita, Image Chat imposta questo valore a 0, il che significa che tutti i commenti sono piatti (nessuna risposta nidificata). Puoi modificarlo se desideri conversazioni threaded.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Consenti 3 livelli di annidamento
});
```

### Configurazione interna

Queste opzioni sono impostate automaticamente da Image Chat e non dovrebbero essere sovrascritte:

Il `productId` è impostato automaticamente su `2` per Image Chat. L'estensione `floating-chat` viene caricata automaticamente per fornire la funzionalità della finestra di chat. Il widget rileva automaticamente i dispositivi mobili (schermi sotto i 768px di larghezza) e adatta l'interfaccia di conseguenza con finestre di chat a schermo intero.

### Flessibilità dell'elemento di destinazione

Il primo parametro di `FastCommentsImageChat` può essere sia un elemento `<img>` direttamente sia un elemento contenitore con un'immagine al suo interno:

```javascript
// Elemento immagine diretto
FastCommentsImageChat(document.getElementById('my-image'), config);

// Contenitore con immagine all'interno
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Il widget troverà automaticamente l'immagine se passi un elemento contenitore.

### Esempio completo

Ecco un esempio che mostra più opzioni di configurazione insieme:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // La tua configurazione SSO
    },
    maxReplyDepth: 1
});
```

Per un elenco completo di tutte le opzioni di configurazione disponibili ereditate dal widget base, vedi la documentazione principale di configurazione di FastComments.