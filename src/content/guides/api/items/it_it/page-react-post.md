[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Aggiunge una reazione a una pagina come utente corrente. Un utente può aggiungere ogni ID di reazione una sola volta: aggiungerlo di nuovo ha esito positivo con il codice `already-reacted` e non modifica il conteggio. Un utente può aggiungere diverse reazioni differenti alla stessa pagina.

Gli ID di reazione sono scelti da te e possono contenere fino a 36 caratteri. La pagina viene creata se non esiste ancora. Passa `title` per impostare o aggiornare il titolo della pagina.

[inline-code-attrs-start title = 'Esempio cURL di Reazione Pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta di Reazione Pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** The reaction id, up to 36 characters. **/
    id: string
    /** Sets the page's title. **/
    title?: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta di Reazione Pagina'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' when the user had already added this reaction. 'react-id-too-long' (HTTP 422) when the id is over 36 characters. Otherwise included on failure. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]