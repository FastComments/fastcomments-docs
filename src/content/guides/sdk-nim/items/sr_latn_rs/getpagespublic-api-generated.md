Nabrojavi stranice za zakupca (tenant). Koristi FChat desktop klijent da popuni svoju listu soba.
Zahteva da `enableFChat` bude true na razrešenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtevaju SSO se filtriraju prema pristupu grupama korisnika koji šalje zahtev.

## Parametri

| Naziv | Tip | Obavezan | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| options | GetPagesPublicOptions | Ne |  |

## Odgovor

Vraća: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Primer

[inline-code-attrs-start title = 'getPagesPublic primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]