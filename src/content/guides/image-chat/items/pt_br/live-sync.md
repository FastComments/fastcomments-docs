### Atualizações em Tempo Real

Image Chat usa conexões WebSocket para sincronizar todas as conversas em tempo real entre todos os usuários conectados. Quando alguém cria um novo marcador, adiciona um comentário ou exclui uma discussão, todos os outros usuários visualizando a mesma imagem veem a atualização imediatamente sem precisar atualizar a página.

### Como a sincronização via WebSocket funciona

Quando você inicializa o Image Chat, o widget estabelece uma conexão WebSocket com os servidores do FastComments. Essa conexão permanece aberta durante a sessão do usuário e fica escutando por atualizações relacionadas à imagem atual.

O sistema WebSocket usa três tipos de mensagens de broadcast para o Image Chat. O evento `new-image-chat` é disparado quando alguém cria um novo marcador na imagem. O evento `image-chat-updated` é disparado quando alguém atualiza uma conversa existente. O evento `deleted-image-chat` é disparado quando alguém exclui um marcador.

### Sistema de ID de Broadcast

Para evitar efeitos de eco em que os usuários veem suas próprias ações sendo retransmitidas de volta para eles, cada atualização inclui um `broadcastId` único. Quando um usuário cria ou atualiza um marcador, seu cliente gera um UUID para essa operação. Quando o WebSocket retransmite a atualização para todos os clientes, o cliente de origem ignora a atualização porque ela corresponde ao seu próprio `broadcastId`.

Isso garante uma interação fluida, onde os usuários veem suas alterações imediatamente na interface sem esperar a viagem de ida e volta pelo servidor, enquanto ainda garante que todos os outros usuários recebam a atualização.

### Resiliência da Conexão

Se a conexão WebSocket cair devido a problemas de rede ou manutenção do servidor, o widget tenta reconectar automaticamente. Durante o período de reconexão, os usuários ainda podem interagir com marcadores existentes, mas não verão atualizações em tempo real de outros usuários até que a conexão seja restabelecida.

Uma vez reconectado, o widget ressincroniza para garantir que nenhuma atualização tenha sido perdida. Isso acontece de forma transparente, sem exigir intervenção do usuário.

### Considerações sobre Largura de Banda

As mensagens WebSocket são leves e contêm apenas as informações essenciais necessárias para sincronizar o estado. Criar um novo marcador normalmente usa menos de 1KB de largura de banda. O sistema também inclui agrupamento inteligente para reduzir a frequência de mensagens durante períodos de alta atividade.

Suas métricas de uso no painel do FastComments rastreiam `pubSubMessageCount` e `pubSubBandwidth` para que você possa monitorar a atividade de sincronização em tempo real em seus sites.

### Sincronização entre abas

Se um usuário tiver a mesma página aberta em várias abas do navegador, as atualizações em uma aba aparecem imediatamente nas outras abas. Isso funciona através do mesmo mecanismo de sincronização por WebSocket e não requer qualquer configuração adicional.

Os usuários podem ter seu site aberto em vários dispositivos simultaneamente, e todos eles permanecerão sincronizados. Um marcador criado em um computador desktop aparece instantaneamente no tablet do usuário se ambos os dispositivos estiverem visualizando a mesma imagem.

### Segurança

As mensagens WebSocket são transmitidas por conexões seguras (WSS) e incluem validação do tenant para garantir que os usuários recebam apenas atualizações das conversas que estão autorizados a ver. O servidor valida todas as operações antes de transmiti-las para evitar acesso ou manipulação não autorizados.

### Comportamento Offline

Quando os usuários estão completamente offline, eles ainda podem visualizar marcadores existentes, mas não podem criar novos nem ver atualizações de outros. O widget detecta o estado offline e exibe mensagens apropriadas.

Se um usuário tentar criar um marcador enquanto estiver offline e depois voltar a ficar online, a operação falhará em vez de ser enfileirada, garantindo consistência dos dados. Os usuários devem tentar novamente a operação assim que a conexão for restaurada.

### Impacto no Desempenho

A conexão WebSocket tem impacto mínimo no desempenho. A conexão permanece ociosa quando não há atualizações ocorrendo e só processa mensagens quando há atividade. Em uma imagem típica com atividade moderada de marcadores, o WebSocket usa menos CPU do que renderizar a própria imagem.

Para páginas com centenas de usuários simultâneos e alta atividade de criação de marcadores, o sistema escala horizontalmente para manter o desempenho sem impactar as conexões dos clientes individuais.

### Casos de Uso Colaborativos

A sincronização em tempo real torna o Image Chat particularmente poderoso para fluxos de trabalho colaborativos. Equipes de design podem revisar mockups juntas, com todos vendo a colocação dos marcadores em tempo real. Equipes de suporte ao cliente podem anotar screenshots colaborativamente para identificar problemas. Grupos educacionais podem discutir diagramas com todos os participantes vendo os marcadores uns dos outros conforme são criados.

O feedback imediato cria uma experiência colaborativa mais envolvente e produtiva em comparação com sistemas de comentários tradicionais, onde os usuários precisam atualizar a página para ver as atualizações.