Page Reacts lässt Ihre Benutzer eine Seite liken oder mit eigenen Reaktionsbildern darauf reagieren. Das [Page Reacts widget](/guide-page-reacts.html) und das Floating Likes-Widget basieren auf diesen Endpunkten, und Sie können sie selbst aufrufen, um Ihren eigenen Like-Button zu erstellen.

Im Gegensatz zum Rest dieses Leitfadens sind die Page Reacts-Endpunkte öffentlich. Sie werden aus den Browsern Ihrer Benutzer aufgerufen, benötigen keinen API-Schlüssel und kosten keine API-Guthaben. Jede Reaktion gehört dem Benutzer, der die Anfrage stellt, sodass ein Benutzer nur seine eigenen hinzufügen oder entfernen kann.

Es gibt zwei Sätze von Endpunkten:

- `/page-reacts/v1/likes/:tenantId` - ein einzelnes „Like“ pro Benutzer pro Seite. Verwenden Sie diese für einen Like-Button.
- `/page-reacts/v2/:tenantId` - mehrere Reaktionen pro Seite, jede identifiziert durch eine kurze `id`, die Sie wählen (z. B. `heart` oder `laugh`).

Beide sind auch in unseren SDKs als Teil der `PublicApi` verfügbar, zum Beispiel `getV1PageLikes`, `createV1PageReact` und `deleteV1PageReact` im [JavaScript SDK](/guide-sdk-javascript.html).

### Identifizierung des Benutzers

Reaktionen sind an den Benutzer gebunden, der die Anfrage stellt:

- **SSO‑Benutzer:** übergeben Sie den `sso`‑Abfrageparameter, gesetzt auf das URI‑kodierte JSON desselben SSO‑Objekts, das Sie dem Kommentar‑Widget geben. Siehe [SSO](/guide-customizations-and-configuration.html#sso).
- **Anonyme Benutzer:** Wenn kein `sso`‑Parameter und keine FastComments‑Anmeldung vorhanden sind, weist der Server dem Browser eine anonyme ID zu, die im FastComments‑Session‑Cookie gespeichert wird. Senden Sie Anfragen mit `credentials: 'include'`, damit das Cookie zwischen den Anfragen erhalten bleibt. Browser, die Drittanbieter‑Cookies blockieren, behalten die anonyme ID nicht, daher sollten Sie SSO verwenden, wenn jeder Benutzer zuverlässig erkannt werden muss.

### Die urlId

`urlId` identifiziert die Seite, genauso wie bei Kommentaren. Verwenden Sie dieselbe `urlId`, die Sie dem Kommentar‑Widget geben, damit Likes und Kommentare auf derselben Seite gezählt werden. Denken Sie daran, sie URI‑zu kodieren.

[inline-code-attrs-start title = 'Beispiel für Like-Button'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optional, für SSO‑Benutzer. Das gleiche Objekt, das Sie dem Kommentar‑Widget‑„sso“-Option geben.
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

---