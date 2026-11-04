[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utilizza la crittografia HMAC-SHA256 come meccanismo per implementare l'SSO. Prima illustreremo l'architettura complessiva, forniremo esempi e passaggi dettagliati.

Esiste anche della documentazione riguardante la migrazione da altri provider con meccanismi SSO simili, e le differenze.

Il flusso è il seguente:

<div class="screenshot white-bg">
    <div class="title">Flusso Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagramma Secure SSO" />
</div>

Poiché Secure SSO coinvolge lo sviluppo full-stack, esempi di codice completi e funzionanti in Java/Spring, NodeJS/Express e PHP vanilla sono attualmente disponibili <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">su GitHub</a>.

Sebbene utilizziamo ExpressJS nell'esempio NodeJS e Spring nell'esempio Java, non sono richiesti framework/librerie in questi runtime per implementare FastComments SSO - i pacchetti crypto nativi funzionano.

Non devi scrivere nuovi endpoint API con FastComments SSO. Crittografa semplicemente le informazioni dell'utente utilizzando la tua chiave segreta e passa il payload al widget dei commenti.

#### Ottieni la Tua Chiave Segreta API

La tua chiave segreta API può essere recuperata da <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">questa pagina</a>. Puoi trovare questa pagina anche andando su Il Mio Account, cliccando sul riquadro API/SSO e poi cliccando su "Get API Secret Key".

#### Parametri del Widget dei Commenti

La documentazione API di alto livello per il widget dei commenti si trova <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">qui</a>.

Entriamo più nel dettaglio sul significato di questi parametri.

Il widget dei commenti accetta un oggetto di configurazione - lo stai già passando se stai utilizzando FastComments per passare il tuo id cliente (chiamato tenantId).

Per abilitare l'SSO, passa un nuovo oggetto "sso", che deve avere i seguenti parametri. I valori devono essere generati lato server.

- userDataJSONBase64: I dati dell'utente in formato JSON, codificati poi in Base64.
- verificationHash: L'hash HMAC-SHA256 creato da UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Timestamp epoch, in **millisecondi**. Non deve essere nel futuro, né più vecchio di due giorni.
- loginURL: Un URL che il widget dei commenti può mostrare per far accedere l'utente.
- logoutURL: Un URL che il widget dei commenti può mostrare per far disconnettere l'utente.
- loginCallback: Quando fornito al posto dell'URL di login, una funzione che il widget dei commenti invocherà al clic sul pulsante di login.
- logoutCallback: Quando fornito al posto dell'URL di logout, una funzione che il widget dei commenti invocherà al clic sul pulsante di logout.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Codice Client Secure SSO'; isFunctional = false; code-example-end]

#### L'Oggetto Utente

L'oggetto Utente contiene il seguente schema:
[inline-code-attrs-start title = 'L\'Oggetto Utente'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Required. 1k Characters Max. **/
    id: string;
    /** Required. 1k Characters Max. Note: Must be unique. **/
    email: string;
    /** Required. 1k Characters Max. Note: The username cannot be an email. Does not have to be unique. **/
    username: string;
    /** Optional. 3k Characters Max for URLs. Default is from gravatar based on email. Supports 64 encoded images, in which case the limit is 50k characters. **/ 
    avatar?: string;
    /** Optional. Default false. **/
    optedInNotifications?: boolean;
    /** Optional. Default false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optional. 100 Characters Max. This label will be shown next to their name. Default is Administrator/Moderator when applicable. **/
    displayLabel?: string;
    /** Optional. 500 Characters Max. This will be shown instead of the username. **/
    displayName?: string;
    /** Optional. 2k Characters Max. The user's name will link to this. **/
    websiteUrl?: string;
    /** Optional. Up to 100 groups per user. A group id may not be longer than 50 characters. **/
    groupIds?: string[];
    /** Optional. Denotes the user as an administrator. **/
    isAdmin?: boolean;
    /** Optional. Denotes the user as a moderator. **/
    isModerator?: boolean;
    /** Optional, default true. Set to false to enable the "activity" tab in the user's profile. **/
    isProfileActivityPrivate?: boolean;
    /** Optional, default false. Set to true to disable profile comments. **/
    isProfileCommentsPrivate?: boolean;
    /** Optional, default false. Set to true to disable direct messaging this user. **/
    isProfileDMDisabled?: boolean;
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of global badge IDs to assign. Limited to 30 badges. Order is respected. **/
        badgeIds: string[];
        /** Array of badge IDs scoped to the current page (urlId). Only displayed on the assigned page. **/
        pageBadgeIds?: string[];
        /** If true, replaces existing displayed badges. Global and page-scoped are overridden independently. **/
        override?: boolean;
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatori e Amministratori

Per amministratori e moderatori, passa i rispettivi flag `isAdmin` o `isModerator` nell'oggetto `SSOUser`.

#### Notifiche

Per abilitare o disabilitare le notifiche, imposta il valore di `optedInNotifications` rispettivamente su `true` o `false`. La prima volta che l'utente carica la pagina con questo valore nel payload SSO, le sue impostazioni di notifica verranno aggiornate.

Inoltre, se vuoi che gli utenti ricevano email di notifica per l'attività sulle pagine a cui si sono iscritti (in contrapposizione alle sole notifiche in-app), imposta `optedInSubscriptionNotifications` su `true`.

#### Utenti VIP ed Etichette Speciali

Puoi visualizzare un'etichetta speciale accanto al nome dell'utente utilizzando il campo opzionale "displayLabel".

#### Utenti non autenticati

Per rappresentare un utente non autenticato, semplicemente non popolare userDataJSONBase64, verificationHash o timestamp. Fornisci un loginURL.

Questi utenti non potranno commentare, e invece verrà mostrato loro un messaggio di login (messaggio, link o pulsante, a seconda della configurazione).

#### Esempi Diretti per la Serializzazione e l'Hashing dei Dati Utente

Maggiori dettagli come esempio <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">qui</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">qui</a> (java) e <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">qui</a> (php).

Comprendiamo che qualsiasi integrazione può essere un processo complicato e doloroso. Non esitare a contattare il tuo referente o a utilizzare la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">pagina di supporto</a>.