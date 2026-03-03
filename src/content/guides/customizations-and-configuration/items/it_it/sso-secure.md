[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utilizza la cifratura HMAC-SHA256 come meccanismo per implementare SSO. Prima descriveremo l'architettura generale, forniremo esempi e passi dettagliati.

Esiste anche della documentazione riguardante la migrazione da altri provider con meccanismi SSO simili, e le differenze.

Il flusso è il seguente:

<div class="screenshot white-bg">
    <div class="title">Flusso Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagramma Secure SSO" />
</div>

Poiché Secure SSO coinvolge sviluppo full-stack, esempi di codice funzionanti completi in Java/Spring, NodeJS/Express e PHP vanilla sono attualmente <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">su GitHub</a>.

Sebbene utilizziamo ExpressJS nell'esempio NodeJS e Spring nell'esempio Java, non sono necessari framework/librerie in questi runtime per implementare FastComments SSO - funzionano i pacchetti crypto nativi.

Non è necessario scrivere nuovi endpoint API con FastComments SSO. Basta crittografare le informazioni dell'utente usando la tua chiave segreta e passare il payload al widget dei commenti.

#### Ottieni la tua chiave segreta API

La tua API Secret può essere recuperata da <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">questa pagina</a>. Puoi trovare questa pagina anche andando su My Account, cliccando il riquadro API/SSO, e poi cliccando "Get API Secret Key".

#### Parametri del widget dei commenti

La documentazione API di alto livello per il widget dei commenti può essere trovata <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">qui</a>.

Entriamo più in dettaglio su cosa significano questi parametri.

Il widget dei commenti prende un oggetto di configurazione - lo passi già se stai usando FastComments per fornire il tuo customer id (chiamato tenantId).

Per abilitare SSO, passa un nuovo oggetto "sso", che deve avere i seguenti parametri. I valori dovrebbero essere generati lato server.

- userDataJSONBase64: I dati dell'utente in formato JSON, poi codificati in Base64.
- verificationHash: L'hash HMAC-SHA256 creato da UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Timestamp epoch, in **millisecondi**. Non deve essere nel futuro, né più di due giorni nel passato.
- loginURL: Un URL che il widget dei commenti può mostrare per effettuare il login dell'utente.
- logoutURL: Un URL che il widget dei commenti può mostrare per effettuare il logout dell'utente.
- loginCallback: Quando fornito al posto del login URL, una funzione che il widget dei commenti invocherà quando si clicca il pulsante di login.
- logoutCallback: Quando fornito al posto del logout URL, una funzione che il widget dei commenti invocherà quando si clicca il pulsante di logout.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User object contains the following schema:
[inline-code-attrs-start title = 'Oggetto utente'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obbligatorio. Massimo 1k caratteri. **/
    id: string;
    /** Obbligatorio. Massimo 1k caratteri. Nota: Deve essere univoco. **/
    email: string;
    /** Obbligatorio. Massimo 1k caratteri. Nota: Lo username non può essere una email. Non deve essere unico. **/
    username: string;
    /** Facoltativo. Massimo 3k caratteri per URL. Predefinito: da gravatar basato sull'email. Supporta immagini codificate in base64, nel qual caso il limite è 50k caratteri. **/ 
    avatar?: string;
    /** Facoltativo. Predefinito false. **/
    optedInNotifications?: boolean;
    /** Facoltativo. Predefinito false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Facoltativo. Massimo 100 caratteri. Questa etichetta sarà mostrata accanto al loro nome. Predefinito: Amministratore/Moderatore quando applicabile. **/
    displayLabel?: string;
    /** Facoltativo. Massimo 500 caratteri. Questo sarà mostrato al posto dello username. **/
    displayName?: string;
    /** Facoltativo. Massimo 2k caratteri. Il nome dell'utente punterà a questo URL. **/
    websiteUrl?: string;
    /** Facoltativo. Fino a 100 gruppi per utente. Un id del gruppo non può essere più lungo di 50 caratteri. **/
    groupIds?: string[];
    /** Facoltativo. Indica che l'utente è amministratore. **/
    isAdmin?: boolean;
    /** Facoltativo. Indica che l'utente è moderatore. **/
    isModerator?: boolean;
    /** Facoltativo, predefinito true. Impostare a false per abilitare la scheda "activity" nel profilo dell'utente. **/
    isProfileActivityPrivate?: boolean;
    /** Facoltativo, predefinito false. Impostare a true per disabilitare i commenti del profilo. **/
    isProfileCommentsPrivate?: boolean;
    /** Facoltativo, predefinito false. Impostare a true per disabilitare i messaggi diretti a questo utente. **/
    isProfileDMDisabled?: boolean;
    /** Configurazione facoltativa per i badge utente. **/
    badgeConfig?: {
        /** Array di ID badge globali da assegnare. Limitato a 30 badge. L'ordine è rispettato. **/
        badgeIds: string[];
        /** Array di ID badge con scope alla pagina corrente (urlId). Visualizzati solo nella pagina assegnata. **/
        pageBadgeIds?: string[];
        /** Se true, sostituisce i badge già visualizzati. I badge globali e quelli a livello di pagina vengono sovrascritti indipendentemente. **/
        override?: boolean;
        /** Se true, aggiorna le proprietà di visualizzazione dei badge dalla configurazione del tenant. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatori e Amministratori

Per amministratori e moderatori, passa i rispettivi flag `isAdmin` o `isModerator` nell'oggetto `SSOUser`.

#### Notifiche

Per abilitare o disabilitare le notifiche, imposta il valore di `optedInNotifications` su `true` o `false` rispettivamente. La prima volta che l'utente carica la pagina con questo valore nel payload SSO, le sue impostazioni di notifica saranno aggiornate.

Inoltre, se vuoi che gli utenti ricevano email di notifica per attività sulle pagine a cui si sono iscritti (anziché solo notifiche in-app), imposta `optedInSubscriptionNotifications` su `true`.

#### Utenti VIP e etichette speciali

Puoi mostrare un'etichetta speciale accanto al nome dell'utente usando il campo opzionale "displayLabel".

#### Utenti non autenticati

Per rappresentare un utente non autenticato, semplicemente non popolare userDataJSONBase64, verificationHash o timestamp. Fornisci un loginURL.

Questi utenti non potranno commentare e invece verranno presentati con un messaggio di login (messaggio, link o pulsante, a seconda della configurazione).

#### Esempi diretti per serializzare e calcolare l'hash dei dati utente

Ulteriori dettagli ed esempi sono disponibili <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">qui</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">qui</a> (java) e <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">qui</a> (php).

Comprendiamo che qualsiasi integrazione può essere un processo complicato e doloroso. Non esitare a contattare il tuo referente o a utilizzare la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">pagina di supporto</a>.