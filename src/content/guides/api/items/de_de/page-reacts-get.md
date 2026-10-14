[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Gibt die Anzahl für jede Reaktion auf einer Seite zurück und welche Reaktionen der aktuelle Benutzer hinzugefügt hat.

Verwenden Sie `includeUserInfo=public`, um auch die Benutzer zu erhalten, die reagiert haben, als einen Eintrag pro Reaktion mit seiner Reaktions‑ID, neueste zuerst. Benutzer mit privater Profilaktivität und anonyme Benutzer werden ausgelassen, und bis zu 100 Reaktionen werden abgefragt. Hier wird nur `public` akzeptiert; `include-private` liefert einen `422` und ist über [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) mit einem API‑Schlüssel verfügbar. Auf Seiten, die SSO erfordern, gibt `includeUserInfo` einen `403` zurück für Benutzer, die die Seite nicht anzeigen dürfen.

[inline-code-attrs-start title = 'Page Reacts cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI‑codiertes JSON Ihres SSO‑Objekts. Für anonyme Benutzer weglassen. **/
    sso?: string
    /** Auf 'public' setzen, um die Benutzer einzuschließen, die reagiert haben. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Bei einem Fehler enthalten. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Bei einem Fehler enthalten. **/
    reason?: string
    /** Anzahl pro Reaktions‑ID, z. B. {"heart": 12, "laugh": 3}. Nicht gesetzt, wenn die Seite keine Reaktionen hat. **/
    counts?: Record<string, number>
    /** Die Reaktions‑IDs, die der anfragende Benutzer hinzugefügt hat. Nicht gesetzt, wenn er keine hinzugefügt hat. **/
    reactedIds?: string[]
    /** Ein Eintrag pro Reaktion, neueste zuerst. Nur gesetzt, wenn includeUserInfo übergeben wird. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** Die Reaktions‑ID, z. B. "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]