### API Overview

Image Chat, görüntü konuşmalarını programlı olarak yönetmek için üç REST API uç noktası sağlar. Bu uç noktalar, tarayıcı widget'ını kullanmadan marker'ları (işaretleri) almanıza, oluşturmanıza ve silmenize olanak tanır.

Tüm API uç noktaları kimlik doğrulama gerektirir ve standart FastComments API desenlerini izler. Bunlar tarayıcı çerezleri aracılığıyla kimlik doğrulayan herkese açık uç noktalardır; API anahtarları kullanılmaz.

### Base URL

Tüm Image Chat API uç noktaları şunun altında bulunur:

```
https://fastcomments.com/comment-image-chats
```

### Authentication

Bu uç noktalar kullanıcıları tarayıcı çerezleriyle doğrular. API anahtarları kullanılmaz. Bu uç noktalara erişebilmek için kullanıcıların tarayıcılarında FastComments'e giriş yapmış olmaları gerekir.

### Get All Conversations

Belirli bir görüntü için tüm görüntü konuşmalarını alın.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (yol parametresi, zorunlu) FastComments Tenant ID'nizdir.

`urlId` (sorgu parametresi, zorunlu) konuşmaları almak istediğiniz görüntü tanımlayıcısıdır.

#### Response

Yanıt, API durumunu, doğrulanmışsa mevcut kullanıcı oturum bilgilerini, kimlikleri, URL'leri ve X/Y koordinatlarıyla birlikte konuşma dizisini, temizlenmiş bir URL tanımlayıcısını, mevcut kullanıcının site yöneticisi veya moderatör olup olmadığını belirten bir bayrağı ve canlı senkronizasyon için `tenantIdWS`, `urlIdWS` ve `userIdWS` dahil WebSocket bağlantı ayrıntılarını içerir.

#### Example Request

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Example Response

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### Create Conversation

Bir görüntü üzerinde belirli bir konum için yeni bir görüntü konuşması oluşturun.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (yol parametresi, zorunlu) FastComments Tenant ID'nizdir.

İstek gövdesi JSON olmalı ve aşağıdaki zorunlu alanları içermelidir.

`urlId` (string, zorunlu) temel sayfa tanımlayıcısıdır.

`windowUrlId` (string, zorunlu) URL'nin, görüntü kaynağı ve koordinatlarla birleşmiş halidir; örneğin `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, zorunlu) sayfanın başlığıdır.

`src` (string, zorunlu) görüntü kaynak URL'sidir.

`x` (number, zorunlu) X koordinatıdır ve yüzde olarak (0-100) ifade edilir.

`y` (number, zorunlu) Y koordinatıdır ve yüzde olarak (0-100) ifade edilir.

`createdFromCommentId` (string, zorunlu) bu sohbeti başlatan yorumun ID'sidir.

`broadcastId` (string, zorunlu) yankı efektlerini önlemek için canlı senkronizasyon amacıyla kullanılan bir UUID'dir.

#### Response

Yanıt, API durumunu ve yeni oluşturulan konuşmanın ID'sini içerir.

#### Example Request

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### Example Response

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Delete Conversation

Mevcut bir görüntü konuşmasını silin. Bu uç nokta yönetici veya moderatör izinleri gerektirir veya konuşma doğrulanmış kullanıcı tarafından oluşturulmuş olmalıdır.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (yol parametresi, zorunlu) FastComments Tenant ID'nizdir.

`chatId` (yol parametresi, zorunlu) silinecek konuşmanın ID'sidir.

`broadcastId` (istek gövdesi, zorunlu) canlı senkronizasyon için bir UUID'dir.

#### Example Request

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Example Response

```json
{
  "status": "success"
}
```

### Coordinate System

X ve Y koordinatları, görüntü boyutlarının yüzdesi olarak saklanır. X, sol kenardan yatay konumu temsil eder (0 = sol kenar, 100 = sağ kenar). Y, üst kenardan dikey konumu temsil eder (0 = üst kenar, 100 = alt kenar).

Bu yüzde değerleri hassasiyet için ondalık değerler içerebilir. Örneğin, x: 25.5 görüntünün sol kenarından %25.5 uzaklıkta olduğu anlamına gelir.

### Rate Limiting

Bu uç noktalar, standart FastComments API hız sınırlamasına tabidir. Hız sınırlama ara yazılımı, kötüye kullanımı önlemek ve normal kullanım desenlerine izin vermek için tenant başına uygulanır.

### Error Responses

Tüm uç noktalar standart HTTP durum kodları döner. 400 yanıtı geçersiz istek parametrelerini gösterir. 401 yanıtı kimlik doğrulamanın başarısız olduğunu belirtir. 403 yanıtı yetersiz izinleri gösterir. 404 yanıtı konuşmanın bulunamadığını belirtir. 429 yanıtı hız sınırının aşıldığını gösterir.

Hata yanıtları şu ayrıntıları içeren bir JSON gövdesi içerir:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Usage Tracking

Konuşma oluşturma, `conversationCreateCount` kullanım metriklerinizi artırır. Tüm WebSocket senkronizasyon etkinlikleri `pubSubMessageCount` ve `pubSubBandwidth` değerlerini artırır. Bu metrikleri FastComments kontrol panelinizde kullanım analizleri altında izleyebilirsiniz.