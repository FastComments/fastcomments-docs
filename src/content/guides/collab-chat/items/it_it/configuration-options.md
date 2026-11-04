### Panoramica

FastComments Collab Chat estende il widget di commenti standard di FastComments, quindi eredita tutte le opzioni di configurazione del widget di base, aggiungendone alcune specifiche per le annotazioni testuali.

### Configurazione Richiesta

#### tenantId

È richiesto il tuo Tenant ID di FastComments. Puoi trovarlo nella tua [dashboard FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Opzioni Specifiche di Collab Chat

#### urlId

Per impostazione predefinita, Collab Chat genera un identificatore univoco per ogni conversazione basato sull'URL della pagina, sul percorso DOM dell'elemento e sull'intervallo di testo selezionato. Puoi sovrascriverlo con un `urlId` personalizzato.

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Questo è utile quando la struttura del tuo URL potrebbe cambiare ma vuoi mantenere le stesse conversazioni, o quando vuoi condividere le annotazioni tra più pagine.

#### topBarTarget

Controlla la visualizzazione della barra superiore che mostra il numero di utenti e di discussioni. Imposta su `null` per disabilitare completamente la barra superiore, oppure fornisci un elemento DOM per renderla in una posizione specifica.

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disabilita la barra superiore
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Renderizza la barra superiore in una posizione personalizzata
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Abilita lo stile della modalità scura quando la tua pagina ha uno sfondo scuro. Questo rilevamento è automatico, ma potrebbe essere desiderabile sovrascriverlo.

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Una funzione di callback che viene eseguita ogni volta che il numero di commenti cambia. Questo è utile per aggiornare elementi dell'interfaccia come badge o titoli di pagina.

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Totale commenti:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Opzioni di Configurazione Ereditate

Poiché Collab Chat estende il widget di commenti standard, puoi usare qualsiasi opzione di configurazione del widget FastComments di base. Ecco alcune opzioni comunemente utilizzate:

#### locale

Imposta la lingua per l'interfaccia del widget. FastComments supporta decine di lingue.

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spagnolo
});
[inline-code-end]

#### readonly

Rende tutte le conversazioni di sola lettura. Gli utenti possono visualizzare le annotazioni esistenti ma non possono crearne di nuove né rispondere.

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso e simpleSSO

Integra il tuo sistema di autenticazione utilizzando il Single Sign-On.

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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

Controlla quanti livelli di profondità possono raggiungere le risposte. Per impostazione predefinita, Collab Chat imposta questo valore su 0, il che significa che tutti i commenti sono piatti (nessuna risposta annidata). Puoi modificarlo se desideri conversazioni con thread.

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Consenti 3 livelli di annidamento
});
[inline-code-end]

### Configurazione Interna

Queste opzioni sono impostate automaticamente da Collab Chat e non dovrebbero essere sovrascritte:

Il `productId` è impostato automaticamente su `3` per Collab Chat. L'estensione `floating-chat` viene caricata automaticamente per fornire la funzionalità della finestra di chat. Il widget rileva automaticamente i dispositivi mobili (schermi con larghezza inferiore a 768px) e adatta l'interfaccia di conseguenza.

### Esempio Completo

Ecco un esempio che mostra più opzioni di configurazione insieme:

[inline-code-attrs-start title = "Esempio di Configurazione"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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

Per un elenco completo di tutte le opzioni di configurazione disponibili ereditate dal widget di base, consulta la documentazione principale di configurazione di FastComments.