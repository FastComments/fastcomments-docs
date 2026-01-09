[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO utilizza la crittografia HMAC-SHA256 come meccanismo per implementare SSO. Per prima cosa vedremo l'architettura generale, forniremo esempi e passaggi dettagliati.

C'è anche della documentazione riguardante la migrazione da altri provider con meccanismi SSO simili e le differenze.

Il flusso è il seguente:

<div class="screenshot white-bg">
    <div class="title">Flusso SSO sicuro</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagramma SSO sicuro" />
</div>

Poiché Secure SSO coinvolge sviluppo full-stack, esempi di codice completi in Java/Spring, NodeJS/Express e PHP vanilla sono attualmente <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">su GitHub</a>.

Anche se usiamo ExpressJS nell'esempio NodeJS e Spring nell'esempio Java, non sono necessari framework/librerie in questi runtime per implementare FastComments SSO - i pacchetti di crittografia nativi funzionano.

Non è necessario scrivere nuovi endpoint API con FastComments SSO. Semplicemente cripta le informazioni dell'utente usando la tua chiave segreta e passa il payload al comment widget.

#### Ottieni la tua chiave API segreta

La tua API Secret può essere recuperata da <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">questa pagina</a>. Puoi trovare questa pagina anche andando in My Account, cliccando la tessera API/SSO e poi cliccando "Get API Secret Key".

#### Parametri del Comment Widget

La documentazione API di alto livello per il comment widget può essere trovata <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">qui</a>.

Entriamo nei dettagli di cosa significano questi parametri.

Il comment widget accetta un oggetto di configurazione - lo passi già se stai usando FastComments per fornire il tuo customer id (chiamato tenantId).

Per abilitare SSO, passa un nuovo oggetto "sso", che deve avere i seguenti parametri. I valori dovrebbero essere generati lato server.

- userDataJSONBase64: I dati dell'utente in formato JSON, poi codificati in Base64.
- verificationHash: L'hash HMAC-SHA256 creato da UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Timestamp epoch, in **millisecondi**. Non deve essere nel futuro, né di oltre due giorni nel passato.
- loginURL: Un URL che il comment widget può mostrare per effettuare il login dell'utente.
- logoutURL: Un URL che il comment widget può mostrare per effettuare il logout dell'utente.
- loginCallback: Quando fornito invece dell'URL di login, una funzione che il comment widget invoca cliccando il pulsante di login.
- logoutCallback: Quando fornito invece dell'URL di logout, una funzione che il comment widget invoca cliccando il pulsante di logout.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### L'oggetto utente

The User object contains the following schema:
[inline-code-attrs-start title = 'Oggetto utente'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obbligatorio. Massimo 1k caratteri. **/
    id: string;
    /** Obbligatorio. Massimo 1k caratteri. Nota: Deve essere univoco. **/
    email: string;
    /** Obbligatorio. Massimo 1k caratteri. Nota: Lo username non può essere un'email. Non deve essere univoco. **/
    username: string;
    /** Opzionale. Massimo 3k caratteri per URL. Il valore predefinito proviene da gravatar basato sull'email. Supporta immagini codificate in base64, nel qual caso il limite è di 50k caratteri. **/ 
    avatar?: string;
    /** Opzionale. Predefinito false. **/
    optedInNotifications?: boolean;
    /** Opzionale. Predefinito false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opzionale. Massimo 100 caratteri. Questa etichetta verrà mostrata accanto al loro nome. Il valore predefinito è Amministratore/Moderatore quando applicabile. **/
    displayLabel?: string;
    /** Opzionale. Massimo 500 caratteri. Verrà mostrato al posto di username. **/
    displayName?: string;
    /** Opzionale. Massimo 2k caratteri. Il nome dell'utente sarà collegato a questo. **/
    websiteUrl?: string;
    /** Opzionale. Fino a 100 gruppi per utente. Un id di gruppo non può essere più lungo di 50 caratteri. **/
    groupIds?: string[];
    /** Opzionale. Indica che l'utente è un amministratore. **/
    isAdmin?: boolean;
    /** Opzionale. Indica che l'utente è un moderatore. **/
    isModerator?: boolean;
    /** Opzionale, predefinito true. Impostare su false per abilitare la scheda "attività" nel profilo dell'utente. **/
    isProfileActivityPrivate?: boolean;
    /** Opzionale, predefinito false. Impostare su true per disabilitare i commenti del profilo. **/
    isProfileCommentsPrivate?: boolean;
    /** Opzionale, predefinito false. Impostare su true per disabilitare i messaggi diretti a questo utente. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderatori e amministratori

Per amministratori e moderatori, passa i rispettivi flag `isAdmin` o `isModerator` nell'oggetto `SSOUser`.

#### Notifiche

Per abilitare o disabilitare le notifiche, imposta il valore di `optedInNotifications` a `true` o `false` rispettivamente. La prima volta che l'utente carica la pagina con questo valore nel payload SSO, le sue impostazioni di notifica verranno aggiornate.

Inoltre, se vuoi che gli utenti ricevano email di notifica per l'attività sulle pagine a cui si sono iscritti (anziché solo notifiche in-app), imposta `optedInSubscriptionNotifications` su `true`.

#### Utenti VIP e etichette speciali

Puoi mostrare un'etichetta speciale accanto al nome dell'utente utilizzando il campo opzionale `displayLabel`.

#### Utenti non autenticati

Per rappresentare un utente non autenticato, semplicemente non popolare userDataJSONBase64, verificationHash o timestamp. Fornisci un loginURL.

Questi utenti non potranno commentare e invece verrà loro mostrato un messaggio di login (messaggio, link o pulsante, a seconda della configurazione).

#### Esempi diretti per la serializzazione e l'hashing dei dati utente

Ulteriori dettagli ed esempi sono disponibili <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">qui</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">qui</a> (java) e <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">qui</a> (php).

Comprendiamo che qualsiasi integrazione può essere un processo complicato e difficile. Non esitare a contattare il tuo referente o usare la <a href="https://fastcomments.com/auth/my-account/help" target="_blank">pagina di supporto</a>.