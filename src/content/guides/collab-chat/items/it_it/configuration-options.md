### Panoramica

FastComments Collab Chat estende il widget di commenti standard di FastComments, ereditando tutte le opzioni di configurazione del widget base e aggiungendo alcune specifiche per le annotazioni di testo.

### Configurazione richiesta

#### tenantId

È richiesto il tuo Tenant ID di FastComments. Puoi trovarlo nella tua [dashboard FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Opzioni specifiche di Collab Chat

#### urlId

Di default, Collab Chat genera un identificatore univoco per ogni conversazione basato sull'URL della pagina, sul percorso DOM dell'elemento e sull'intervallo di testo selezionato. Puoi sovrascriverlo con un `urlId` personalizzato.

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Questo è utile quando la struttura del tuo URL potrebbe cambiare ma vuoi mantenere le stesse conversazioni, o quando vuoi condividere le annotazioni tra più pagine.

#### topBarTarget

Controlla la visualizzazione della barra superiore che mostra il conteggio utenti e il conteggio delle discussioni. Impostalo su `null` per disabilitare completamente la barra superiore, oppure fornisci un elemento DOM per renderizzarla in una posizione specifica.

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disabilita la barra superiore
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Mostra la barra superiore in una posizione personalizzata
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Abilita lo stile in modalità scura quando la tua pagina ha uno sfondo scuro. Questa rilevazione è automatica, ma potrebbe essere desiderabile sovrascriverla.

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Una funzione di callback che viene eseguita ogni volta che il conteggio dei commenti cambia. È utile per aggiornare elementi dell'interfaccia utente come badge o il titolo della pagina.

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Opzioni di configurazione ereditate

Poiché Collab Chat estende il widget di commenti standard, puoi usare qualsiasi opzione di configurazione del widget base di FastComments. Ecco alcune opzioni comunemente usate:

#### locale

Imposta la lingua per l'interfaccia del widget. FastComments supporta decine di lingue.

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spagnolo
});
[inline-code-end]

#### readonly

Rendi tutte le conversazioni in sola lettura. Gli utenti possono visualizzare le annotazioni esistenti ma non possono crearne di nuove o rispondere.

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Integra con il tuo sistema di autenticazione usando il Single Sign-On.

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Configurazione SSO
    }
});
[inline-code-end]

Consulta la documentazione SSO per i dettagli completi sulle opzioni di autenticazione.

#### maxReplyDepth

Controlla quanti livelli di profondità possono avere le risposte. Di default, Collab Chat imposta questo valore a 0, il che significa che tutti i commenti sono piatti (nessuna risposta annidata). Puoi modificarlo se desideri conversazioni threadate.

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Consenti 3 livelli di annidamento
});
[inline-code-end]

### Configurazione interna

Queste opzioni sono impostate automaticamente da Collab Chat e non dovrebbero essere sovrascritte:

Il `productId` è impostato automaticamente su `3` per Collab Chat. L'estensione `floating-chat` viene caricata automaticamente per fornire la funzionalità della finestra di chat. Il widget rileva automaticamente i dispositivi mobili (schermi con larghezza inferiore a 768px) e adatta l'interfaccia di conseguenza.

### Esempio completo

Ecco un esempio che mostra più opzioni di configurazione insieme:

[inline-code-attrs-start title = "Esempio di configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // La tua configurazione SSO
    },
    maxReplyDepth: 1
});
[inline-code-end]

Per un elenco completo di tutte le opzioni di configurazione disponibili ereditate dal widget base, consulta la documentazione principale di configurazione di FastComments.