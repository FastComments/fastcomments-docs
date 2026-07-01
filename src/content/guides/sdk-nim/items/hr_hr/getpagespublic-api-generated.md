Popis stranica za zakupca. Koristi ga FChat desktop klijent za popunjavanje popisa soba.  
Zahtijeva da je `enableFChat` postavljen na true u razriješenoj prilagođenoj konfiguraciji za svaku stranicu.  
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji zahtijeva.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetPagesPublicOptions | No |  |

## Response

Returns: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]