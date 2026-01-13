### Atualizações em Tempo Real

Collab Chat usa conexões WebSocket para sincronizar todas as conversas em tempo real entre todos os usuários conectados. Quando alguém cria uma nova anotação, adiciona um comentário ou exclui uma discussão, todos os outros usuários visualizando a mesma página veem a atualização imediatamente sem precisar atualizar.

### Como a Sincronização via WebSocket Funciona

Quando você inicializa o Collab Chat, o widget estabelece uma conexão WebSocket com os servidores do FastComments. Essa conexão permanece aberta durante a sessão do usuário e escuta por atualizações relacionadas à página atual.

O sistema WebSocket usa três tipos de mensagens de broadcast para o Collab Chat. O evento `new-text-chat` é disparado quando alguém cria uma nova anotação na página. O evento `updated-text-chat` é disparado quando alguém atualiza uma conversa existente. O evento `deleted-text-chat` é disparado quando alguém exclui uma anotação.

### Sistema de ID de Broadcast

Para evitar efeitos de eco onde os usuários veem suas próprias ações sendo transmitidas de volta para eles, cada atualização inclui um `broadcastId` único. Quando um usuário cria ou atualiza uma anotação, seu cliente gera um UUID para essa operação. Quando o WebSocket retransmite a atualização para todos os clientes, o cliente de origem ignora a atualização porque ela corresponde ao seu próprio `broadcastId`.

Isso garante uma interação suave em que os usuários veem suas alterações imediatamente na interface sem esperar a ida e volta pelo servidor, enquanto ainda assegura que todos os outros usuários recebam a atualização.

### Contagem de Usuários em Tempo Real

A barra superior exibe o número de usuários atualmente visualizando a página. Essa contagem é atualizada em tempo real conforme os usuários entram e saem. A contagem de usuários é fornecida pela mesma conexão WebSocket e incrementa/decrementa automaticamente com base nos eventos de conexão e desconexão.

### Resiliência de Conexão

Se a conexão WebSocket cair devido a problemas de rede ou manutenção do servidor, o widget tenta se reconectar automaticamente. Durante o período de reconexão, os usuários ainda podem interagir com as anotações existentes, mas não verão atualizações em tempo real de outros usuários até que a conexão seja restabelecida.

Uma vez reconectado, o widget ressincroniza para garantir que nenhuma atualização tenha sido perdida. Isso acontece de forma transparente sem exigir intervenção do usuário.

### Considerações sobre Largura de Banda

As mensagens WebSocket são leves e contêm apenas as informações essenciais necessárias para sincronizar o estado. Criar uma nova anotação normalmente usa menos de 1KB de largura de banda. O sistema também inclui agrupamento inteligente para reduzir a frequência de mensagens durante períodos de alta atividade.

Suas métricas de uso no painel do FastComments acompanham `pubSubMessageCount` e `pubSubBandwidth` para que você possa monitorar a atividade de sincronização em tempo real em seus sites.

### Sincronização entre Abas

Se um usuário tiver a mesma página aberta em múltiplas abas do navegador, as atualizações em uma aba aparecem imediatamente nas outras. Isso funciona através do mesmo mecanismo de sincronização via WebSocket e não requer nenhuma configuração adicional.

### Segurança

As mensagens WebSocket são transmitidas por conexões seguras (WSS) e incluem validação do tenant para garantir que os usuários recebam apenas atualizações de conversas que estão autorizados a ver. O servidor valida todas as operações antes de retransmiti-las para prevenir acesso ou manipulação não autorizados.