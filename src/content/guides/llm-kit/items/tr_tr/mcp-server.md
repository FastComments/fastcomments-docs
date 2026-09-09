FastComments, AI asistanlarının ve ajan istemcilerinin FastComments API'sini doğrudan çağırabilmesi için barındırılan bir Model Context Protocol (MCP) sunucusu çalıştırır. MCP sunucusunun sunduğu her araç, genel OpenAPI spesifikasyonundan otomatik olarak oluşturulur, bu yüzden REST API'sinin yapabildiği her şey bir MCP istemcisi tarafından yapılabilir.

Uç nokta, durum bilgisiz ve akışa uygun HTTP tabanlıdır. Canlı tutmak için bir oturum yoktur ve istemci başına sunucu tarafı durum bilgisi bulunmaz.

### Uç Nokta

[inline-code-attrs-start title = 'MCP Uç Noktası'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### OAuth ile Bağlan

OAuth (Claude, ChatGPT, Claude Code, Cursor ve diğerleri) ile uzaktan sunucuları destekleyen herhangi bir MCP istemcisi, FastComments tarafında herhangi bir kurulum yapmadan yukarıdaki uç noktaya bağlanabilir. İstemci, Dinamik İstemci Kaydı aracılığıyla kendini kaydeder veya bir Client ID Metadata Document ile kimliğini belirler, bir tarayıcı açar ve FastComments'a oturum açıp erişimi onaylamanızı sağlar, ve oturum açtığınız hesaba bağlı bir token alır.

Keşif belgeleri standart konumlarda bulunur:

[inline-code-attrs-start title = 'Keşif'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Kullanıcınızın bir bağlantıyı onaylamak için hesapta API Yönetici iznine sahip olması gerekir. Birden fazla hesabı yönetiyorsanız, onaylamadan önce kontrol panelinde doğru hesaba geçiş yapın.

Bir istemci `read` kapsamını, `write` kapsamını veya her ikisini isteyebilir. Hiçbir şey istemeyen bir istemci her ikisini de alır. Veri değiştiren araçlar yalnızca okuma izni olan token'a sunulmaz.

Kontrol panelinde, yapıştırmaya hazır snippet'ler içeren bir kurulum yardımcısı bulunur. **Integrate -> MCP Server**'ı açın veya doğrudan ziyaret edin:

[inline-code-attrs-start title = 'Kurulum Sayfası'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

FastComments sunucusunu tek bir komutla kaydedin, ardından bir oturum içinde `/mcp` komutunu çalıştırarak oturum açın ve kullanılabilir araçları listeleyin:

[inline-code-attrs-start title = 'Claude Code Kurulumu'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor ve diğer yapılandırma dosyası istemcileri

Bu bloğu istemcinizin MCP sunucuları yapılandırmasına ekleyin (`mcp.json` Cursor için). İstemci, ilk kullanımda oturum açmak için bir tarayıcı açar.

[inline-code-attrs-start title = 'MCP İstemci Yapılandırması'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### Erişimi İptal Etme

Kontrol panelinde **Integrate -> Connected Apps** altında onaylanmış her bağlantı listelenir. Birini iptal etmek, o uygulamanın tuttuğu tüm token'ları geçersiz kılar. Uygulamalar bağlandıklarında kendilerini kaydeder ve FastComments bunları incelemez, bu yüzden tanımadığınız her şeyi iptal edin.

### Token'ı REST API ile Kullanma

Bir MCP istemcisinin elde ettiği erişim token'ı, normal bir FastComments API kimliğidir. `/api/v1` uç noktasının her birinde taşıyıcı token olarak çalışır, bu yüzden MCP üzerinden bağlanan bir uygulama REST API'yi doğrudan da çağırabilir:

[inline-code-attrs-start title = 'Taşıyıcı Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Kiracı (tenant), token tarafından ima edilir. Bir `tenantId` hâlâ gönderilebilir ancak eşleşmelidir. `GET` istekleri `read` kapsamına, diğer tüm istekler `write` kapsamına ihtiyaç duyar.

### API Anahtarı ile Bağlan

Tarayıcı oturumu tamamlayamayan istemciler, örneğin başsız sunucular, bunun yerine bir API anahtarıyla kimlik doğrulaması yapabilir. `tenantId` ve `API_KEY`'i sorgu parametreleri olarak gönderin veya istemciniz özel başlıkları destekliyorsa `x-tenant-id` ve `x-api-key` HTTP başlıkları olarak gönderin:

[inline-code-attrs-start title = 'API Anahtarı Uç Noktası'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Kurulum sayfası, her bir API anahtarınız için bu URL'yi oluşturur.

### Güvenlik

API anahtarı içeren bir uç nokta URL'si bir sırdır: bunu herkese açık sohbetlere, ekran görüntülerine veya commit'lere yapıştırmayın. Bir anahtar ifşa olursa, kontrol panelinizdeki API Anahtarları sayfasından yenileyin. OAuth token'ları böyle bir risk taşımaz çünkü tek bir uygulamaya bağlanmıştır ve Bağlı Uygulamalardan iptal edilebilir.