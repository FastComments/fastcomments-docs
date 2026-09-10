Webhooks ayrıca REST API üzerinden yönetilebilir. Bu, Zapier gibi entegrasyonların kontrol paneline dokunmadan yorum olaylarına abone olmasını sağlar ve REST Hooks desenini izler: abone ol, olayları al, aboneliği iptal et.

API abonelikleri, kontrol panelinde yapılandırılan webhooks'larla birlikte bulunur. Bir yorum olayı, kendi alanı için kontrol paneli webhook'una ve eşleşen her API aboneliğine ayrı ayrı teslim edilir. Olay başına bir abone limiti yoktur.

## Kimlik Doğrulama

Her istek, `x-api-key` başlığında (veya `API_KEY` sorgu parametresinde) API Anahtarınızı ve `tenantId` sorgu parametresinde kiracı kimliğinizi gerektirir. İkisi de kontrol panelindeki API Gizli Sayfası'nda gösterilir.

## Abone Ol

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Alan | Gerekli | Açıklama |
|-------|----------|-------------|
| `url` | Evet | Mutlak bir http veya https URL'si. |
| `event` | Evet | `comment-created`, `comment-updated` veya `comment-deleted`. |
| `domain` | Hayır | Hesap yapılandırmanızdaki bir domain. Varsayılan `*`, tüm domain'ler için olayları alır. |
| `method` | Hayır | `POST` (varsayılan), `PUT` veya `DELETE`. |

Yanıt, aboneliği içerir:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Aynı URL'yi aynı olay ve domain'e tekrar abone etmek, bir kopya oluşturmak yerine mevcut aboneliği döndürür, bu yüzden bir istemci güvenle yeniden deneyebilir. Her kiracı en fazla 50 API aboneliğine sahip olabilir.

## Liste

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Kiracı için tüm webhook'ları döndürür, kontrol panelinde yönetilenler dahil (`"source": "dashboard"`). `event`, `domain` veya `source` ile filtreleyin.

## Aboneliği İptal Et

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Bir aboneliği silmek, ona hâlâ kuyruğa alınmış olan tüm olayları da iptal eder. Yalnızca API üzerinden oluşturulan abonelikler bu şekilde silinebilir. Kontrol paneli webhook'ları Webhooks sayfasında düzenlenir.

## Yükler ve İmzalama

Teslimatlar, kontrol paneli webhook'larıyla aynı yükü kullanır (Data Structures bölümüne bakın) ve aynı HMAC şemasıyla imzalanır (Security & API Tokens bölümüne bakın). API abonelikleri asla eski `token` başlığını almaz, bu yüzden `X-FastComments-Signature` başlığını doğrulayın.

## 410 Gone Yanıtı

Bir API aboneliğinin uç noktası HTTP `410 Gone` yanıtı verirse, FastComments bunu bir abonelik iptali olarak değerlendirir: abonelik, kuyruğa alınmış olaylarıyla birlikte silinir ve başka teslimat denenmez. Kontrol panelinde yapılandırılan webhook'lar otomatik olarak silinmez; onlar için 410 sıradan bir hatadır. Diğer tüm hata durumları yeniden denenir ve sonunda webhook devre dışı bırakılır, How it Works & Handling Retries bölümünde açıklandığı gibi.

## Kontrol Paneli

API abonelikleri, oluşturuldukları domain'in altında Webhooks sayfasında listelenir; burada bir yönetici onları devre dışı bırakabilir, yeniden etkinleştirebilir veya silebilir.