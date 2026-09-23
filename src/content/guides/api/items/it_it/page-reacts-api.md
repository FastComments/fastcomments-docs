Page Reacts consente ai tuoi utenti di mettere mi piace a una pagina o di reagire ad essa con il tuo set di immagini di reazione. Il widget [Page Reacts widget](/guide-page-reacts.html) e il widget Floating Likes sono basati su questi endpoint e puoi chiamarli direttamente per creare il tuo pulsante mi piace.

A differenza del resto di questa guida, gli endpoint di Page Reacts sono pubblici. Vengono chiamati dal browser dei tuoi utenti, non richiedono una chiave API e non consumano crediti API. Ogni reazione appartiene all'utente che effettua la richiesta, quindi un utente può aggiungere o rimuovere solo le proprie.

Ci sono due gruppi di endpoint:

- `/page-reacts/v1/likes/:tenantId` – un singolo “mi piace” per utente per pagina. Usali per un pulsante mi piace.  
- `/page-reacts/v2/:tenantId` – più reazioni per pagina, ciascuna identificata da un breve `id` che scegli (ad esempio `heart` o `laugh`).

Entrambi sono disponibili anche nei nostri SDK come parte della `PublicApi`, ad esempio `getV1PageLikes`, `createV1PageReact` e `deleteV1PageReact` nel [JavaScript SDK](/guide-sdk-javascript.html).

### Identifying the User

Le reazioni sono legate all'utente che effettua la richiesta:

- **SSO users:** passa il parametro di query `sso`, impostato sul JSON codificato in URI dello stesso oggetto SSO che fornisci al widget dei commenti. Vedi [SSO](/guide-customizations-and-configuration.html#sso).  
- **Anonymous users:** quando non è presente il parametro `sso` e non c'è un login FastComments, il server assegna al browser un ID anonimo memorizzato nel cookie di sessione FastComments. Invia le richieste con `credentials: 'include'` affinché il cookie venga mantenuto tra le richieste. I browser che bloccano i cookie di terze parti non conserveranno l'ID anonimo, quindi usa SSO quando ogni utente deve essere riconosciuto in modo affidabile.

### The urlId

`urlId` identifica la pagina, come avviene per i commenti. Usa lo stesso `urlId` che fornisci al widget dei commenti affinché i mi piace e i commenti vengano conteggiati sulla stessa pagina. Ricorda di codificarlo in URI.

[inline-code-attrs-start title = 'Esempio di Pulsante Mi Piace'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Opzionale, per gli utenti SSO. Lo stesso oggetto che fornisci all'opzione "sso" del widget dei commenti.
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