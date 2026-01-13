O FastComments SDK fornece dois tipos de endpoints de API:

### PublicAPI - Client-Safe Endpoints

O `PublicAPI` contém endpoints que são seguros para serem chamados a partir de código do lado do cliente (aplicativos iOS/macOS). Esses endpoints:
- Não requerem uma chave de API
- Podem usar tokens SSO para autenticação
- Têm limites de taxa por usuário/dispositivo
- São adequados para aplicações voltadas ao usuário final

**Exemplo de caso de uso**: Buscar e criar comentários no seu aplicativo iOS

### DefaultAPI - Server-Side Endpoints

O `DefaultAPI` contém endpoints autenticados que requerem uma chave de API. Esses endpoints:
- Requerem sua chave de API do FastComments
- Devem SER chamados APENAS a partir de código do lado do servidor
- Fornecem acesso completo aos seus dados do FastComments
- Têm limites de taxa por tenant

**Exemplo de caso de uso**: Operações administrativas, exportação em massa de dados, ferramentas de moderação

**IMPORTANTE**: Nunca exponha sua chave de API em código do lado do cliente. Chaves de API devem ser usadas apenas no servidor.