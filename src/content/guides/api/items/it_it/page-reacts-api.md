Page Reacts consente ai tuoi utenti di mettere mi piace a una pagina o di reagire ad essa con il proprio set di immagini di reazione. Il [Page Reacts widget](/guide-page-reacts.html) e il widget Floating Likes sono basati su questi endpoint e puoi chiamarli tu stesso per creare il tuo pulsante mi piace.

A differenza del resto di questa guida, gli endpoint di Page Reacts sono pubblici. Vengono chiamati dai browser dei tuoi utenti, non richiedono una chiave API e non consumano crediti API. Ogni reazione appartiene all'utente che effettua la richiesta, quindi un utente può aggiungere o rimuovere solo le proprie.

Esistono due set di endpoint:

- `/page-reacts/v1/likes/:tenantId` - un singolo "like" per utente per pagina. Usa questi per un pulsante mi piace.
- `/page-reacts/v2/:tenantId` - reazioni multiple per pagina, ciascuna identificata da un breve `id` che scegli (ad esempio `heart` o `laugh`).

Entrambi sono disponibili anche nei nostri SDK come parte della `PublicApi`, ad esempio `getV1PageLikes`, `createV1PageReact` e `deleteV1PageReact` nel [JavaScript SDK](/guide-sdk-javascript.html).

### Visualizzare chi ha reagito

Passa `includeUserInfo=public` agli endpoint `GET` per ottenere anche gli utenti che hanno messo mi piace o reagito, con i loro nomi utente e avatar, dal più recente al più vecchio. Gli utenti che hanno impostato la loro attività del profilo come privata vengono esclusi, così come gli utenti anonimi. Vengono restituiti fino a 100 reazioni.

Gli utenti SSO vengono creati con attività del profilo privata a meno che il tuo payload SSO imposti `isProfileActivityPrivate` su `false`, quindi impostalo per gli utenti che desideri far apparire in queste liste.

Nelle pagine che richiedono SSO, solo gli utenti SSO che possono visualizzare la pagina possono elencare chi ha reagito. Gli altri ricevono un `403` con il codice `sso-required` o `page-forbidden`.

Dal tuo server, [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) e [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) restituiscono le stesse liste con una chiave API e accettano anche `includeUserInfo=include-private` per includere gli utenti con profili privati.

### Identificare l'utente

Reacts are tied to the user making the request:

- **SSO users:** passa il parametro di query `sso`, impostato sul JSON codificato in URI dello stesso oggetto SSO che fornisci al widget dei commenti. Vedi [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonymous users:** quando non c'è il parametro `sso` e nessun login FastComments, il server assegna al browser un id anonimo memorizzato nel cookie di sessione FastComments. Invia le richieste con `credentials: 'include'` affinché il cookie venga mantenuto tra le richieste. I browser che bloccano i cookie di terze parti non manterranno l'id anonimo, quindi usa SSO quando ogni utente deve essere riconosciuto in modo affidabile.

### L'urlId

`urlId` identifica la pagina, come avviene per i commenti. Usa lo stesso `urlId` che fornisci al widget dei commenti in modo che i mi piace e i commenti vengano conteggiati sulla stessa pagina. Ricorda di codificarlo in URI.

[inline-code-attrs-start title = 'Esempio di pulsante Mi piace'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optional, for SSO users. The same object you give the comment widget's "sso" option.
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