### API Genel Bakış

Collab Chat, metin konuşmalarını programlı olarak yönetmek için üç REST API uç noktası sağlar. Bu uç noktalar, tarayıcı widget'ını kullanmadan açıklamaları (annotations) almanıza, oluşturmanıza ve silmenize olanak tanır.

Bu uç noktalar, kullanıcıları tarayıcı çerezleri aracılığıyla kimlik doğrular. API anahtarları kullanılmaz. Kullanıcıların bu uç noktalara erişmek için tarayıcılarında FastComments'a giriş yapmış olmaları gerekir.

### Temel URL

Tüm Collab Chat API uç noktaları şunun altındadır:

[inline-code-attrs-start title = 'Temel URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Kimlik Doğrulama

Bu uç noktalar, kullanıcıları tarayıcı çerezleri aracılığıyla kimlik doğrular. API anahtarları kullanılmaz. Kullanıcıların bu uç noktalara erişmek için tarayıcılarında FastComments'a giriş yapmış olmaları gerekir.

### Tüm Konuşmaları Getir

Belirli bir sayfa için tüm metin konuşmalarını alın.

#### Uç Nokta

[inline-code-attrs-start title = 'GET Uç Noktası'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametreler

`tenantId` (yol parametresi, gerekli) FastComments Tenant ID'nizdir.

`urlId` (sorgu parametresi, gerekli) konuşmaları almak istediğiniz sayfanın tanımlayıcısıdır.

#### Yanıt

Yanıt, API durumunu, kimlik doğrulandıysa geçerli kullanıcı oturum bilgilerini, ID'leri, URL'leri ve metin aralıkları ile bir konuşmalar dizisini, temizlenmiş bir URL tanımlayıcısını, geçerli kullanıcının site yöneticisi veya moderatör olup olmadığını gösteren bir bayrağı ve canlı senkronizasyon için `tenantIdWS`, `urlIdWS` ve `userIdWS` dahil olmak üzere WebSocket bağlantı ayrıntılarını içerir.

#### Örnek İstek

[inline-code-attrs-start title = 'GET İstek Örneği'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Örnek Yanıt

[inline-code-attrs-start title = 'GET Yanıt Örneği'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### Konuşma Oluştur

Belirli bir metin seçimi için yeni bir metin konuşması oluşturun.

#### Uç Nokta

[inline-code-attrs-start title = 'POST Uç Noktası'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametreler

`tenantId` (yol parametresi, gerekli) FastComments Tenant ID'nizdir.

İstek gövdesi JSON olmalı ve aşağıdaki gerekli alanları içermelidir.

`urlId` (string, gerekli) sayfanın temel tanımlayıcısıdır.

`urlIdWithRange` (string, gerekli) URL ile metin aralığının birleşimidir; örneğin `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, gerekli) sayfanın başlığıdır.

`selector` (string, gerekli) seçili metni içeren elementi gösteren DOM yoludur.

`range` (string, gerekli) `startOffset:endOffset,startOffset:endOffset{checksum}` formatında serileştirilmiş metin aralığıdır.

`createdFromCommentId` (string, gerekli) bu sohbeti başlatan yorumun ID'sidir.

`broadcastId` (string, gerekli) yankı etkilerini önlemek için canlı senkronizasyon amacıyla kullanılan bir UUID'dir.

#### Yanıt

Yanıt, API durumunu ve yeni oluşturulan konuşmanın ID'sini içerir.

#### Örnek İstek

[inline-code-attrs-start title = 'POST İstek Örneği'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### Örnek Yanıt

[inline-code-attrs-start title = 'POST Yanıt Örneği'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Konuşmayı Sil

Mevcut bir metin konuşmasını silin. Bu uç nokta yönetici veya moderatör izinleri gerektirir veya konuşma kimlik doğrulanan kullanıcı tarafından oluşturulmuş olmalıdır.

#### Uç Nokta

[inline-code-attrs-start title = 'DELETE Uç Noktası'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametreler

`tenantId` (yol parametresi, gerekli) FastComments Tenant ID'nizdir.

`chatId` (yol parametresi, gerekli) silinecek konuşmanın ID'sidir.

`broadcastId` (istek gövdesi, gerekli) canlı senkronizasyon için bir UUID'dir.

#### Örnek İstek

[inline-code-attrs-start title = 'DELETE İstek Örneği'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Örnek Yanıt

[inline-code-attrs-start title = 'DELETE Yanıt Örneği'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Oran Sınırlandırması

Bu uç noktalar standart FastComments API oran sınırlarına tabidir. Oran sınırı ara yazılımı, normal kullanım desenlerine izin verirken kötüye kullanımı önlemek için kiracı başına uygulanır.

### Hata Yanıtları

Tüm uç noktalar standart HTTP durum kodlarını döner. 400 yanıtı geçersiz istek parametrelerini gösterir. 401 yanıtı kimlik doğrulamanın başarısız olduğunu belirtir. 403 yanıtı yetersiz izinleri gösterir. 404 yanıtı konuşmanın bulunamadığını belirtir. 429 yanıtı ise oran sınırının aşıldığını gösterir.

Hata yanıtları ayrıntıları içeren bir JSON gövdesi içerir:

[inline-code-attrs-start title = 'Hata Yanıtı Örneği'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Kullanım Takibi

Konuşma oluşturmak `conversationCreateCount` kullanım metriğinizi artırır. Tüm WebSocket senkronizasyon etkinliği `pubSubMessageCount` ve `pubSubBandwidth` değerlerini artırır. Bu metrikleri FastComments kontrol panelinizde kullanım analizleri altında izleyebilirsiniz.

---