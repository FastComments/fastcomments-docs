Page Reacts pozwala Twoim użytkownikom polubić stronę lub zareagować na nią przy użyciu własnego zestawu obrazków reakcji. Widżet [Page Reacts](/guide-page-reacts.html) oraz widżet Floating Likes są zbudowane na tych endpointach i możesz wywoływać je samodzielnie, aby stworzyć własny przycisk polubienia.

W przeciwieństwie do reszty tego przewodnika, endpointy Page Reacts są publiczne. Są wywoływane z przeglądarek Twoich użytkowników, nie wymagają klucza API i nie kosztują kredytów API. Każda reakcja należy do użytkownika, który wykonał żądanie, więc użytkownik może dodawać lub usuwać tylko własne reakcje.

Istnieją dwa zestawy endpointów:

- `/page-reacts/v1/likes/:tenantId` – pojedyncze „polubienie” na użytkownika na stronę. Użyj tego dla przycisku polubienia.
- `/page-reacts/v2/:tenantId` – wiele reakcji na stronę, każda identyfikowana krótkim `id`, które wybierzesz (np. `heart` lub `laugh`).

Oba są również dostępne w naszych SDK jako część `PublicApi`, na przykład `getV1PageLikes`, `createV1PageReact` i `deleteV1PageReact` w [JavaScript SDK](/guide-sdk-javascript.html).

### Identifying the User

Reakcje są powiązane z użytkownikiem wykonującym żądanie:

- **Użytkownicy SSO:** przekaż parametr zapytania `sso`, ustawiony na zakodowany w URI JSON tego samego obiektu SSO, który podajesz widżetowi komentarzy. Zobacz [SSO](/guide-customizations-and-configuration.html#sso).
- **Użytkownicy anonimowi:** gdy nie ma parametru `sso` i nie ma logowania FastComments, serwer przydziela przeglądarce anonimowy identyfikator przechowywany w ciasteczku sesji FastComments. Wysyłaj żądania z `credentials: 'include'`, aby ciasteczko było zachowane między żądaniami. Przeglądarki blokujące ciasteczka third‑party nie zachowają anonimowego identyfikatora, więc używaj SSO, gdy każdy użytkownik musi być wiarygodnie rozpoznany.

### The urlId

`urlId` identyfikuje stronę, tak samo jak w komentarzach. Użyj tego samego `urlId`, które podajesz widżetowi komentarzy, aby polubienia i komentarze były liczone na tej samej stronie. Pamiętaj, aby zakodować go w URI.

[inline-code-attrs-start title = 'Przykład przycisku polubienia'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Opcjonalnie, dla użytkowników SSO. Ten sam obiekt, który przekazujesz opcji \"sso\" widżetu komentarzy.
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