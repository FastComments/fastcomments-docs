Izpiše strani za najemnika. Uporablja namizni odjemalec FChat za polnjenje svojega seznama sob.
Zahteva, da je `enableFChat` nastavljen na true v razrešeni prilagojeni konfiguraciji vsake strani.
Strani, ki zahtevajo SSO, so filtrirane glede na skupinski dostop zahtevajočega uporabnika.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| options | GetPagesPublicOptions | Ne |  |

## Odgovor

Vrne: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]