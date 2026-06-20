O SDK do FastComments fornece três clientes de API:

### PublicAPI - Métodos seguros para cliente

O `PublicAPI` contém métodos seguros para serem chamados a partir de código do lado do cliente (apps iOS/macOS). Esses métodos:
- Não requerem uma API key
- Podem usar tokens SSO para autenticação
- São rate-limited por usuário/dispositivo
- São adequados para aplicações voltadas ao usuário final

**Exemplo de uso**: Buscar e criar comentários em seu app iOS

### DefaultAPI - Métodos do lado do servidor

O `DefaultAPI` contém métodos autenticados que requerem uma API key. Esses métodos:
- Requerem sua FastComments API key
- Devem SER CHAMADOS SOMENTE a partir de código do lado do servidor
- Fornecem acesso completo aos seus dados do FastComments
- São rate-limited por tenant

**Exemplo de uso**: Operações administrativas, exportação em massa de dados, gerenciamento de usuários

### ModerationAPI - Métodos do Painel de Moderador

O `ModerationAPI` contém métodos que dão suporte ao painel do moderador. Esses métodos abrangem:
- **Moderação de comentários** - listar, contar, buscar, recuperar logs e exportar comentários
- **Ações de moderação** - remover/restaurar comentários, sinalizar, definir status de revisão/spam/aprovação, gerenciar votos e reabrir/fechar tópicos
- **Banimentos** - banir um usuário de um comentário, desfazer banimentos, buscar resumos pré-banimento, verificar status de banimento e preferências, e ler contagens de usuários banidos
- **Insígnias & confiança** - conceder/remover insígnias, listar insígnias manuais, obter/definir o fator de confiança de um usuário, e ler o perfil interno de um usuário

Todo método do `ModerationAPI` aceita um parâmetro `sso` para que moderadores possam ser autenticados via SSO.

**Exemplo de uso**: Construir uma experiência de moderação para os moderadores da sua comunidade

**IMPORTANTE**: Nunca exponha sua API key em código do lado do cliente. API keys devem ser usadas apenas no lado do servidor.