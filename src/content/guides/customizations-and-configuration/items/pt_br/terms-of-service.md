FastComments permite que você exija que comentaristas pela primeira vez aceitem seus Termos de Serviço antes de enviar um comentário.

When enabled:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### Configuração

Navegue até a página de personalização do widget e habilite a caixa de seleção "Require Terms of Service acceptance". Uma vez habilitada, você verá as seguintes opções:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: Por padrão, a caixa de seleção exibe "I agree to the Terms of Service and Privacy Policy" com links para ambos os documentos. Selecione "Customize text per locale" para fornecer seu próprio texto para cada idioma.
- **TOS Last Updated Date**: Quando você atualizar seus Termos de Serviço, defina essa data. Usuários que aceitaram antes dessa data serão obrigados a aceitar novamente.

### Como Funciona

- O carimbo de data/hora de aceitação dos TOS é armazenado por usuário e por comentário
- Quando um usuário aceita os TOS, a data é registrada em seu perfil de usuário (por tenant)
- Se você definir uma data de "Last Updated" que seja posterior à data de aceitação do usuário, ele precisará aceitar novamente
- Para usuários anônimos que não podem ser rastreados, a caixa de seleção aparece em cada envio de comentário