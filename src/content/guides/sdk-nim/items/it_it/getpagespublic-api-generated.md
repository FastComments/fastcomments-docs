Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare la sua lista delle stanze.  
Richiede `enableFChat` impostato su true nella configurazione personalizzata risolta per ogni pagina.  
Le pagine che richiedono SSO sono filtrate in base all'accesso ai gruppi dell'utente richiedente.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| options | GetPagesPublicOptions | No |  |

## Risposta

Restituisce: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]