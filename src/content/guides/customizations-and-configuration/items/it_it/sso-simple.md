[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Con Simple SSO, possiamo fornire al widget di commenti le informazioni sull'utente in modo che non debba inserire nome utente o email per commentare.

Possiamo configurare Simple SSO come segue:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

L'utente verrà autenticato e verrà creato un utente SSO in background. L'utente avrà `createdFromSimpleSSO` impostato a `true` se recuperato dall'API.

Note: 

- L'email è l'identificatore univoco per Simple SSO.
- Fornire un'email con Simple SSO non è obbligatorio, tuttavia per impostazione predefinita i loro commenti verranno mostrati come "Unverified". <b>Se non viene fornita un'email, l'utente non può essere completamente autenticato.</b>
- **NUOVO** Dal gennaio 2022: i nomi utente non devono essere unici su tutto fastcomments.com
- Simple SSO può creare e aggiornare automaticamente gli utenti SSO, se viene fornita un'email e l'utente non è stato originariamente creato da Secure SSO. Se un payload Simple SSO indica un utente creato da Secure SSO o dall'API, quell'utente rimane invariato e il visitatore ottiene un utente separato.
- È possibile specificare i badge per l'utente con la proprietà `badgeConfig`. L'array `badgeIds` contiene gli ID dei badge globali da associare all'utente. L'array `pageBadgeIds` contiene gli ID dei badge limitati alla pagina corrente (`urlId`) — questi badge vengono visualizzati solo nella pagina in cui sono stati assegnati. Se `override` è impostato a `true`, sostituirà i badge visualizzati esistenti (i badge globali e quelli limitati alla pagina vengono sovrascritti indipendentemente); se `false` verranno aggiunti ai badge esistenti.

#### Impostazioni Simple SSO

Simple SSO è sempre disponibile. Due impostazioni controllano quanto FastComments si fida di esso. È possibile trovarle cercando SSO in una <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regola di personalizzazione</a>. Entrambe sono disattivate per impostazione predefinita e si applicano indipendentemente dal fatto che "Require SSO To View Comments" sia attivo.

Gli account che già utilizzavano Simple SSO quando queste impostazioni sono state aggiunte hanno entrambe attivate, quindi nulla è cambiato per loro.

Solo la regola di personalizzazione che corrisponde meglio a una pagina viene applicata, quindi se hai più di una regola, imposta queste su ciascuna di esse. Una nuova regola inizia con esse attive se una delle tue altre regole le ha attive.

##### Verifica Automatica dei Commenti tramite Simple SSO

Quando è attiva, i commenti degli utenti Simple SSO vengono verificati immediatamente.

Quando è disattiva, i commenti degli utenti Simple SSO vengono mostrati come "Unverified" finché l'utente non clicca un link che inviamo all'indirizzo presente nel payload Simple SSO. Se approvi automaticamente solo i commenti verificati, anche questi attendono quel link. Se non c'è alcuna email nel payload, il commento rimane non verificato.

Questa impostazione influisce solo sui commenti. I voti degli utenti Simple SSO sono sempre conteggiati immediatamente.

##### Consenti Amministratori Simple SSO

Quando è attiva, un utente Simple SSO può essere amministratore o moderatore del tuo account:

- Un utente Simple SSO che ha permessi di amministratore o moderatore li mantiene.
- Un utente Simple SSO con la stessa email di uno dei tuoi amministratori o moderatori acquisisce i loro permessi.

Quando è disattiva, Simple SSO non concede questi permessi. Un payload Simple SSO che indica un utente che li possiede ottiene un utente separato senza permessi, e l'utente originale rimane invariato.

I payload Simple SSO non sono firmati, quindi quando è attiva, chiunque conosca l'email o l'ID di un amministratore può agire come lui. Se questo è importante per il tuo sito, lascia questa opzione disattivata e utilizza Secure SSO per i tuoi amministratori.

Gli utenti creati da Secure SSO o dall'API non possono essere raggiunti da Simple SSO, indipendentemente dal fatto che questa opzione sia attiva o meno.