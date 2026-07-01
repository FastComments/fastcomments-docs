Listanje stranica za tenant. Koristi FChat desktop klijent za popunjavanje njegove liste soba.  
Potrebno je da `enableFChat` bude postavljeno na true u razriješenoj prilagođenoj konfiguraciji za svaku stranicu.  
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu traženog korisnika.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | GetPagesPublicOptions | Ne |  |

## Odgovor

Vraća: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Primjer

[inline-code-attrs-start title = 'getPagesPublic Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]