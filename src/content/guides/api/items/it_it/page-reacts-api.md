I Page Reacts permettono ai tuoi utenti di mettere "mi piace" a una pagina, o di reagire ad essa con il tuo set di immagini di reazione. Il [widget Page Reacts](/guide-page-reacts.html) e il widget Floating Likes sono costruiti su questi endpoint, e puoi chiamarli tu stesso per creare il tuo pulsante "mi piace".

A differenza del resto di questa guida, gli endpoint Page Reacts sono pubblici. Vengono chiamati dai browser dei tuoi utenti, non richiedono una chiave API e non consumano crediti API. Ogni reazione appartiene all'utente che effettua la richiesta, quindi un utente può solo aggiungere o rimuovere le proprie.

Esistono due set di endpoint:

- `/page-reacts/v1/likes/:tenantId` - un singolo "mi piace" per utente per pagina. Usali per un pulsante "mi piace".
- `/page-reacts/v2/:tenantId` - più reazioni per pagina, ognuna identificata da un breve `id` che scegli tu (ad esempio `heart` o `laugh`).

Entrambi sono disponibili anche nei nostri SDK come parte di `PublicApi`, ad esempio `getV1PageLikes`, `createV1PageReact` e `deleteV1PageReact` nello [SDK JavaScript](/guide-sdk-javascript.html).

### Mostrare Chi Ha Reagito

Passa `includeUserInfo=public` agli endpoint `GET` per ottenere anche gli utenti che hanno messo "mi piace" o reagito, con i loro nomi utente e avatar, i più recenti per primi. Gli utenti che hanno reso privata l'attività del proprio profilo sono esclusi, così come gli utenti anonimi. Vengono restituite fino a 100 reazioni.

Gli utenti SSO vengono creati con attività del profilo privata a meno che il tuo payload SSO non imposti `isProfileActivityPrivate` su `false`, quindi impostalo per gli utenti che vuoi far comparire in questi elenchi.

Sulle pagine che richiedono SSO, solo gli utenti SSO che possono visualizzare la pagina possono vedere chi ha reagito. Gli altri ricevono un `403` con il codice `sso-required` o `page-forbidden`.

Dal tuo server, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) e [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) restituiscono gli stessi elenchi con una chiave API, e accettano anche `includeUserInfo=include-private` per includere gli utenti con profili privati.

### Identificare l'Utente

Le reazioni sono legate all'utente che effettua la richiesta:

- **Utenti SSO:** passa il parametro di query `sso`, impostato sul JSON URI encoded dello stesso oggetto SSO che dai al widget dei commenti. Vedi [SSO](/guide-customizations-and-configuration.html#sso).
- **Utenti anonimi:** quando non c'è un parametro `sso` e nessun login FastComments, il server assegna al browser un id anonimo memorizzato nel cookie di sessione FastComments. Invia le richieste con `credentials: 'include'` così il cookie viene mantenuto tra le richieste. I browser che bloccano i cookie di terze parti non manterranno l'id anonimo, quindi usa SSO quando ogni utente deve essere riconosciuto in modo affidabile.

### Il urlId

`urlId` identifica la pagina, allo stesso modo in cui lo fa per i commenti. Usa lo stesso `urlId` che dai al widget dei commenti così che i "mi piace" e i commenti siano conteggiati sulla stessa pagina. Ricorda di fare l'URI encoding.

[inline-code-attrs-start title = 'Esempio di Pulsante Mi Piace'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Opzionale, per utenti SSO. Lo stesso oggetto che dai all'opzione "sso" del widget dei commenti.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]