FastComments permite que você exija que comentaristas pela primeira vez aceitem seus Termos de Serviço antes de enviar um comentário.

Quando ativado:
- **Usuários anônimos** verão a caixa de seleção dos Termos de Serviço toda vez que comentarem
- **Usuários autenticados** verão a caixa de seleção apenas no primeiro comentário, ou quando você atualizar seus Termos de Serviço

### Habilitando os Termos de Serviço

Navegue até a página de personalização do widget e ative a caixa "Exigir aceitação dos Termos de Serviço":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Personalizando o texto dos Termos de Serviço

Por padrão, a caixa exibe "Concordo com os Termos de Serviço e a Política de Privacidade" com links para ambos os documentos. Você pode personalizar esse texto por localidade, se necessário:

1. Selecione "Personalizar texto por localidade"
2. Selecione a localidade no menu suspenso e insira seu texto personalizado

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Atualizando seus Termos de Serviço

Ao atualizar seus Termos de Serviço, defina a data "Última atualização". Usuários que aceitaram os Termos de Serviço antes desta data serão obrigados a aceitar novamente:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Como funciona

- O carimbo de data/hora da aceitação dos Termos de Serviço é armazenado por usuário e por comentário
- Quando um usuário aceita os Termos de Serviço, a data é registrada no perfil do usuário (por locatário)
- Se você definir uma data "Última atualização" posterior à data de aceitação do usuário, ele precisará aceitar novamente
- Para usuários anônimos que não podem ser rastreados, a caixa de seleção aparece em cada envio de comentário