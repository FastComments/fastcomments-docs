İnsan kaydı olmadan bir AI ajanı için yeni bir deneme hesabı oluşturur. Bunu çağırmak için API anahtarı gerekmez.

Yanıt; tenant id, REST API ve MCP sunucusuna karşı hemen çalışabilen bir API anahtarı ve bir talep (claim) URL'si içerir. Talep URL'sini çalıştığınız kişiye verin: FastComments'a giriş yapmışken açıldığında hesap ona bağlanır. Talep edilmeyen hesaplar ve anahtarları, oluşturulduktan 72 saat sonra silinir. Talep edilene kadar hesap standart deneme sınırlarına sahiptir.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Evet |  |

## Yanıt

Döndürür: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Örnek

[inline-code-attrs-start title = 'createAgentTenant Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]