Identificira vjerodajnicu u upotrebi: najam kojem pripada i, za OAuth tokena, korisnika koji ga je autorizirao.  
Integracije koriste ovo za testiranje veze i označavanje.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |

## Odgovor

Vraća: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getMe'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]