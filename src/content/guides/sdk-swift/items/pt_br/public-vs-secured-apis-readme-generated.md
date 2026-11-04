O SDK do FastComments fornece dois tipos de endpoints de API:

### PublicAPI - Endpoints Seguros para o Cliente

A `PublicAPI` contém endpoints que são seguros para serem chamados a partir de código do lado do cliente (aplicativos iOS/macOS). Esses endpoints:
- Não exigem uma chave de API
- Podem usar tokens SSO para autenticação
- São limitados por taxa por usuário/dispositivo
- São adequados para aplicativos voltados ao usuário final

**Exemplo de caso de uso**: Buscar e criar comentários em seu aplicativo iOS

### DefaultAPI - Endpoints do Lado do Servidor

A `DefaultAPI` contém endpoints autenticados que exigem uma chave de API. Esses endpoints:
- Exigem sua chave de API do FastComments
- Devem ser chamados APENAS a partir de código do lado do servidor
- Fornecem acesso completo aos seus dados do FastComments
- São limitados por taxa por tenant

**Exemplo de caso de uso**: Operações administrativas, exportação de dados em massa, ferramentas de moderação

**IMPORTANTE**: Nunca exponha sua chave de API em código do lado do cliente. As chaves de API devem ser usadas apenas no lado do servidor.