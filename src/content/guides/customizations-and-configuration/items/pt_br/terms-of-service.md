FastComments permite que você exija que os comentaristas de primeira vez aceitem seus Termos de Serviço antes de enviar um comentário.

Quando ativado:
- **Usuários anônimos** verão uma caixa de seleção de TOS toda vez que comentarem
- **Usuários autenticados** verão a caixa de seleção apenas no primeiro comentário, ou quando você atualizar seus TOS

### Configuration

Navegue até a página de personalização do widget e habilite a caixa de seleção "Exigir aceitação dos Termos de Serviço". Uma vez habilitado, você verá as seguintes opções:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Painel de Termos de Serviço mostrando o seletor de modo de texto do TOS e o campo de data da última atualização'; title='Opções de Termos de Serviço' app-screenshot-end]

- **Modo de Texto do TOS**: Por padrão, a caixa de seleção exibe "Concordo com os Termos de Serviço e a Política de Privacidade" com links para ambos os documentos. Selecione "Personalizar texto por localidade" para fornecer seu próprio texto para cada idioma.
- **Data da Última Atualização do TOS**: Quando você atualizar seus Termos de Serviço, defina esta data. Usuários que aceitaram antes desta data serão obrigados a aceitar novamente.

### How It Works

- O carimbo de data/hora de aceitação do TOS é armazenado por usuário e por comentário
- Quando um usuário aceita o TOS, a data é registrada em seu perfil de usuário (por locatário)
- Se você definir uma data de "Última Atualização" que seja posterior à data de aceitação do usuário, ele precisará aceitar novamente
- Para usuários anônimos que não podem ser rastreados, a caixa de seleção aparece em cada envio de comentário