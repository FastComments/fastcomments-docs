FastComments runs a hosted Model Context Protocol (MCP) server so AI coding assistants and agentic clients can call the FastComments API directly. Every tool the MCP server exposes is auto-generated from the public OpenAPI spec, so anything the REST API can do, an MCP client can do.

Uç nokta durumsuzdur ve akış (streamable) tabanlı HTTP kullanır. Tutulması gereken bir oturum yoktur, istemci kayıt adımı yoktur ve istemci başına sunucu tarafında durum tutulmaz.

### Endpoint

[inline-code-attrs-start title = 'MCP Uç Noktası'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Kimlik doğrulama, REST API ile aynı API anahtarını kullanır. İstemciniz özel başlıkları destekliyorsa, `tenantId` ve anahtarı `x-tenant-id` ve `x-api-key` HTTP başlıkları olarak da iletebilirsiniz.

### Pre-filled setup

Panoda, popüler MCP istemcileri için URL ve kopyalanmaya hazır yapılandırma parçacıkları (config snippets) üreten bir kurulum yardımcı aracı vardır. Hesap kontrol panelinize gidin ve **Entegre Et -> MCP Sunucusu** öğesini açın veya doğrudan şu adrese gidin:

[inline-code-attrs-start title = 'Kurulum Sayfası'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Kullanmak istediğiniz API anahtarını açılır menüden seçin, ardından oluşturulan parçalardan (snippet) herhangi birini kopyalayın.

### Claude Code

FastComments sunucusunu tek bir komutla kaydedin:

[inline-code-attrs-start title = 'Claude Code Kurulumu'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Kaydedildikten sonra bağlantıyı doğrulamak ve kullanılabilir araçları listelemek için bir Claude Code oturumu içinde `/mcp` komutunu çalıştırın.

### Claude Desktop / Cursor

Bu bloğu istemcinizin MCP sunucuları yapılandırmasına ekleyin (`claude_desktop_config.json` Claude Desktop için, `mcp.json` Cursor için):

[inline-code-attrs-start title = 'MCP İstemci Yapılandırması'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY"
    }
  }
}
[inline-code-end]

### Security

API anahtarı URL içinde gömülüdür. URL'yi bir gizli bilgi gibi ele alın: herkese açık sohbetlere, ekran görüntülerine veya commit'lere yapıştırmayın. Eğer bir anahtar ifşa olursa, panelinizdeki API Anahtarları sayfasından onu değiştirin.