### API Genel Bakış

Image Chat, görüntü konuşmalarını programatik olarak yönetmek için üç REST API endpoint'i sağlar. Bu endpoint'ler, tarayıcı widget'ını kullanmadan marker'ları almanıza, oluşturmanıza ve silmenize olanak tanır.

Tüm API endpoint'leri kimlik doğrulaması gerektirir ve standart FastComments API desenlerini takip eder. Bunlar, API anahtarları yerine tarayıcı çerezleri aracılığıyla kimlik doğrulaması yapan herkese açık endpoint'lerdir.

### Temel URL

Tüm Image Chat API endpoint'leri şunun altındadır:

```
https://fastcomments.com/comment-image-chats
```

### Kimlik Doğrulama

Bu endpoint'ler kullanıcıların kimliğini tarayıcı çerezleri aracılığıyla doğrular. API anahtarları kullanmazlar. Bu endpoint'lere erişmek için kullanıcıların tarayıcılarında FastComments'e giriş yapmış olmaları gerekir.

### Tüm Konuşmaları Getir

Belirli bir görüntü için tüm görüntü konuşmalarını alın.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parametreler

`tenantId` (yol parametresi, zorunlu) sizin FastComments Tenant ID'nizdir.

`urlId` (sorgu parametresi, zorunlu) konuşmalarını almak istediğiniz görüntü tanımlayıcısıdır.

#### Yanıt

Yanıt, API durumunu, kimlik doğrulama yapılmışsa geçerli kullanıcı oturum bilgilerini, ID'leri, URL'leri ve X/Y koordinatlarıyla birlikte konuşmalar dizisini, temizlenmiş bir URL tanımlayıcısını, geçerli kullanıcının site yöneticisi veya moderatör olup olmadığını belirten bir bayrağı ve `tenantIdWS`, `urlIdWS` ve `userIdWS` dahil canlı senkronizasyon için WebSocket bağlantı ayrıntılarını içerir.

#### Örnek İstek

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Örnek Yanıt

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

### Konuşma Oluştur

Bir görüntü üzerinde belirli bir konum için yeni bir görüntü konuşması oluşturun.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parametreler

`tenantId` (yol parametresi, zorunlu) sizin FastComments Tenant ID'nizdir.

İstek gövdesi JSON olmalı ve şu zorunlu alanları içermelidir.

`urlId` (string, zorunlu) temel sayfa tanımlayıcısıdır.

`windowUrlId` (string, zorunlu) görüntü kaynağı ve koordinatlarla birleştirilmiş URL'dir, örneğin `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, zorunlu) sayfanın başlığıdır.

`src` (string, zorunlu) görüntü kaynak URL'sidir.

`x` (number, zorunlu) yüzde (0-100) olarak X koordinatıdır.

`y` (number, zorunlu) yüzde (0-100) olarak Y koordinatıdır.

`createdFromCommentId` (string, zorunlu) bu sohbeti başlatan yorumun ID'sidir.

`broadcastId` (string, zorunlu) yankı etkilerini önlemek için canlı senkronizasyona yönelik bir UUID'dir.

#### Yanıt

Yanıt, API durumunu ve yeni oluşturulan konuşmanın ID'sini içerir.

#### Örnek İstek

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

#### Örnek Yanıt

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Konuşma Sil

Mevcut bir görüntü konuşmasını silin. Bu endpoint yönetici veya moderatör izinleri gerektirir ya da konuşmanın kimliği doğrulanmış kullanıcı tarafından oluşturulmuş olması gerekir.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parametreler

`tenantId` (yol parametresi, zorunlu) sizin FastComments Tenant ID'nizdir.

`chatId` (yol parametresi, zorunlu) silinecek konuşmanın ID'sidir.

`broadcastId` (istek gövdesi, zorunlu) canlı senkronizasyon için bir UUID'dir.

#### Örnek İstek

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Örnek Yanıt

```json
{
  "status": "success"
}
```

### Koordinat Sistemi

X ve Y koordinatları, görüntü boyutlarının yüzdeleri olarak saklanır. X, sol kenardan yatay konumu temsil eder (0 = sol kenar, 100 = sağ kenar). Y, üst kenardan dikey konumu temsil eder (0 = üst kenar, 100 = alt kenar).

Bu yüzde değerleri, hassasiyet için ondalık sayılar içerebilir. Örneğin, x: 25.5, görüntünün sol kenarından %25,5 anlamına gelir.

### Hız Sınırlandırma

Bu endpoint'ler standart FastComments API hız sınırlandırmaya tabidir. Hız sınırı middleware'ı, normal kullanım desenlerine izin verirken kötüye kullanımı önlemek için kiracı (tenant) başına uygulanır.

### Hata Yanıtları

Tüm endpoint'ler standart HTTP durum kodlarını döndürür. 400 yanıtı geçersiz istek parametrelerini gösterir. 401 yanıtı kimlik doğrulamasının başarısız olduğu anlamına gelir. 403 yanıtı yetersiz izinleri gösterir. 404 yanıtı konuşmanın bulunamadığı anlamına gelir. 429 yanıtı hız sınırının aşıldığını gösterir.

Hata yanıtları, ayrıntılar içeren bir JSON gövdesi içerir:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Kullanım Takibi

Konuşma oluşturmak, `conversationCreateCount` kullanım metriğinizi artırır. Tüm WebSocket senkronizasyon etkinliği `pubSubMessageCount` ve `pubSubBandwidth` değerlerini artırır. Bu metrikleri FastComments kontrol panelinizde kullanım analitiği altında izleyebilirsiniz.