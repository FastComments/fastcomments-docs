[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Doda reakcijo na stran kot trenutni uporabnik. Uporabnik lahko doda vsak ID reakcije le enkrat: ponoven dodatek uspe z kodo `already-reacted` in ne spremeni števila. Uporabnik lahko doda več različnih reakcij na isto stran.

ID-ji reakcij jih izberete vi in so lahko dolgi do 36 znakov. Stran se ustvari, če še ne obstaja. Posredujte `title`, da nastavite ali posodobite naslov strani.

[inline-code-attrs-start title = 'Primer cURL zahteve za reakcijo na stran'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za reakcijo na stran'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** ID reakcije, do 36 znakov. **/
    id: string
    /** Nastavi naslov strani. **/
    title?: string
    /** URI kodiran JSON vašega SSO objekta. Izpustite za anonimne uporabnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za reakcijo na stran'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted', ko je uporabnik že dodal to reakcijo. 'react-id-too-long' (HTTP 422), ko je ID daljši od 36 znakov. V nasprotnem primeru je vključen pri napaki. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Vključeno pri napaki. **/
    reason?: string
}
[inline-code-end]

---