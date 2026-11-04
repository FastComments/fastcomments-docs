O FastComments permite exigir que comentaristas de primeira viagem aceitem seus Termos de Serviço antes de enviar um comentário.

Quando ativado:
- **Usuários anônimos** verão uma caixa de seleção dos TOS toda vez que comentarem
- **Usuários autenticados** verão a caixa de seleção apenas no primeiro comentário, ou quando você atualizar seus TOS

### Configuração

Navegue até a página de personalização do widget e ative a caixa de seleção "Require Terms of Service acceptance". Uma vez ativada, você verá as seguintes opções:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Painel de Termos de Serviço mostrando o seletor de modo de texto dos TOS e o campo da data da última atualização'; title='Opções de Termos de Serviço' app-screenshot-end]

- **TOS Text Mode**: Por padrão, a caixa de seleção exibe "I agree to the Terms of Service and Privacy Policy" com links para ambos os documentos. Selecione "Customize text per locale" para fornecer seu próprio texto para cada idioma.
- **TOS Last Updated Date**: Quando você atualizar seus Termos de Serviço, defina esta data. Usuários que aceitaram antes desta data serão obrigados a aceitar novamente.

### Como Funciona

- O timestamp de aceitação dos TOS é armazenado por usuário e por comentário
- Quando um usuário aceita os TOS, a data é registrada no perfil do usuário (por tenant)
- Se você definir uma data de "Last Updated" posterior à data de aceitação do usuário, ele precisará aceitar novamente
- Para usuários anônimos que não podem ser rastreados, a caixa de seleção aparece em cada envio de comentário