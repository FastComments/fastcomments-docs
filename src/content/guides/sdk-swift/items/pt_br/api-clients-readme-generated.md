O FastComments SDK fornece três clientes de API:

### PublicAPI - Métodos Seguros para Cliente

A `PublicAPI` contém métodos que são seguros para chamar a partir de código cliente (apps iOS/macOS). Esses métodos:
- Não exigem uma chave de API
- Podem usar tokens SSO para autenticação
- São limitados por taxa por usuário/dispositivo
- São adequados para aplicações voltadas ao usuário final

**Caso de uso**: Buscar e criar comentários no seu app iOS

### DefaultAPI - Métodos Lado do Servidor

A `DefaultAPI` contém métodos autenticados que exigem uma chave de API. Esses métodos:
- Exigem sua chave de API FastComments
- Devem SER CHAMADOS SOMENTE a partir de código do lado do servidor
- Fornecem acesso total aos seus dados FastComments
- São limitados por taxa por locatário

**Caso de uso**: Operações administrativas, exportação em massa de dados, gerenciamento de usuários

### ModerationAPI - Métodos do Painel de Moderação

A `ModerationAPI` fornece um amplo conjunto de APIs de moderação ao vivo e rápidas. Cada método `ModerationAPI` aceita um parâmetro `sso` e pode autenticar via SSO ou um cookie de sessão FastComments.com.

**Caso de uso**: Construir uma experiência de moderação para moderadores da sua comunidade

**IMPORTANTE**: Nunca exponha sua chave de API no código cliente. As chaves de API devem ser usadas apenas no lado do servidor.