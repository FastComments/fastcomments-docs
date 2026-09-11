Webhooks ayrıca REST API üzerinden yönetilebilir. Bu, Zapier gibi entegrasyonların kontrol paneline dokunmadan yorum olaylarına abone olmasını sağlar ve REST Hooks desenini izler: abone ol, olayları al, aboneliği iptal et.

API abonelikleri, kontrol panelinde yapılandırılmış webhooks'ların yanında bulunur. Bir yorum olayı, alanına uyan her webhook'a, webhook'un nasıl oluşturulduğuna bakılmaksızın, ayrı bir teslimat olarak gönderilir.

## Kimlik Doğrulama

Her istek, `x-api-key` başlığında (veya `API_KEY` sorgu parametresinde) API Anahtarınızı ve `tenantId` sorgu parametresinde kiracı kimliğinizi gerektirir. Her ikisi de kontrol panelindeki API Gizli Sayfası'nda gösterilir.

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
|------|---------|----------|
| `url` | Evet | Mutlak bir http veya https URL'si. |
| `event` | Evet | `comment-created`, `comment-updated` or `comment-deleted`. |
| `domain` | Hayır | Hesap yapılandırmanızdaki bir alan adı. Varsayılan `*` olup, her alan adı için olayları alır. |
| `method` | Hayır | `POST` (default), `PUT` or `DELETE`. |

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

Aynı URL'yi aynı olay ve alan adına tekrar abone etmek, bir kopya oluşturmak yerine mevcut aboneliği döndürür, böylece bir istemci güvenle yeniden deneyebilir. Her kiracı en fazla 50 API aboneliğine sahip olabilir.

## Liste

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Kiracı için kontrol panelinde yönetilenler dahil olmak üzere tüm webhook'ları döndürür (`"source": "dashboard"`). `event`, `domain` veya `source` ile filtreleyin.

## Aboneliği İptal Et

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Bir aboneliği silmek, ona hâlâ kuyrukta bekleyen olayları da iptal eder. Bu şekilde yalnızca API üzerinden oluşturulan abonelikler silinebilir; kontrol paneli webhook'u veya hesabınızda bulunmayan bir kimlik, `404` yanıtını `not-found` koduyla verir. Kontrol paneli webhook'ları Webhooks sayfasında düzenlenir.

## Yükler ve İmzalama

Teslimatlar, kontrol paneli webhook'larıyla aynı yükü kullanır (Data Structures bölümüne bakın) ve aynı HMAC şemasıyla imzalanır (Security & API Tokens bölümüne bakın). API abonelikleri asla eski `token` başlığını almaz, bu yüzden `X-FastComments-Signature` başlığını doğrulayın.

## Örnek Yükler

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Hesabın en son yorumlarını, teslimatın taşıdığı tam biçimde döndürür, böylece bir entegrasyon ilk olay gelmeden gerçek örnek verileri gösterebilir. `event` isteğe bağlıdır ve sadece doğrulanır, çünkü her olay aynı yorum nesnesini taşır. `limit` varsayılan olarak 3'tür ve 1 ile 10 arasında kabul eder. 2 API kredisi maliyetlidir.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## 410 Gone Yanıtı

Bir API aboneliğinin uç noktası HTTP `410 Gone` yanıtı verirse, FastComments bunu bir abonelik iptali olarak değerlendirir: abonelik, kuyrukta bekleyen olaylarıyla birlikte silinir ve başka teslimat denenmez. Kontrol panelinde yapılandırılmış webhook'lar otomatik olarak silinmez; onlar için 410 sıradan bir hatadır. Diğer tüm hata durumları yeniden denenir ve sonunda webhook devre dışı bırakılır, How it Works & Handling Retries bölümünde açıklandığı gibi.

## Kontrol Paneli

API abonelikleri, Webhooks listesinde **API** kaynağıyla görünür; burada bir yönetici onları düzenleyebilir, devre dışı bırakabilir, yeniden etkinleştirebilir veya silebilir.