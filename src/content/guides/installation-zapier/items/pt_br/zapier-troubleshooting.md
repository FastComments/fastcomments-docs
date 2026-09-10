## Solução de Problemas

**"Você não tem permissão" ao conectar.** O usuário conectado não é um administrador da API na conta.  
Peça ao proprietário da conta que conceda permissão de API na página Usuários, ou conecte-se como proprietário.

**A conexão está rotulada com o site errado.** A página de consentimento conecta a conta na qual você estava conectado no momento. Desconecte no Zapier, troque de conta no painel do FastComments e conecte novamente.

**Os eventos pararam de chegar.** Verifique a página Webhooks no painel. Uma assinatura cujo endpoint ficou falhando por seis dias é desativada automaticamente e mostra o motivo. Reative-a lá, ou desligue e ligue o Zap novamente. Se a assinatura estiver totalmente ausente, alguém a excluiu; desligar e ligar o Zap a recria.

**O Zapier informa que a conta precisa ser reconectada.** A conexão foi revogada na página Aplicativos Conectados, o usuário que a aprovou perdeu a permissão de API, ou a conta foi excluída. Reconecte a partir do Zapier.

**Uma ação falha com "não tem acesso de gravação".** A conexão foi aprovada com permissão somente leitura. Reconecte e aprove ambas as permissões.

**Limites de taxa e créditos.** Ações e buscas consomem créditos de API do seu plano e estão sujeitas aos mesmos limites de taxa da API REST. Gatilhos não consomem nenhum. Um Zap que atinge um limite é reexecutado pelo Zapier após o atraso que o FastComments relata.

**O menu suspenso Domínio está vazio.** Domínios aparecem assim que são configurados na página Domínios no painel do FastComments. Deixe o campo em branco para receber eventos de todos os domínios.