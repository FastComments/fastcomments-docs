---
Para desenvolvimento local, use uma ferramenta como o [ngrok](https://ngrok.com/).

Para simplificar a manutenção da segurança do sistema, o desenvolvimento local segue o mesmo processo de configuração e proteção de outros ambientes. 

### Etapa 1: Adicionar "localhost" aos domínios na sua conta.

Adicione "localhost" [como um domínio aqui](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='O formulário de adição de domínio nas configurações da conta com localhost inserido no campo de nomes de domínio'; title='Adicionar localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Etapa 2: Escolher uma chave de API

Vamos adicionar a configuração de webhook para o seu domínio, portanto precisaremos de uma chave de API. [Você pode fazer isso aqui.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Novo formulário de segredo de API com o domínio associado definido como localhost e a chave nomeada Testing'; title='Adicionar chave de API de teste'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Em "Associar ao domínio" - selecione o seu domínio "localhost".

**NOTA: Alternativamente, você pode usar um único Segredo de API para toda a atividade de teste e ambientes de staging. Basta adicionar um Segredo de API para "Todos os Domínios" e dar a ele um nome como "test".**

Certifique-se de que você tem um Segredo de API definido para seu(s) domínio(s) de produção. Eventos para todos os outros domínios usarão o segredo curinga (de teste).

### Etapa 3: Adicionar seu webhook

Enquanto o ngrok ou ferramenta similar estiver em execução, defina o valor para "localhost" [aqui](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Administração de webhooks com o domínio localhost selecionado e uma URL ngrok preenchida no endpoint de criação de comentário'; title='Adicionar webhook de teste'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Ao clicar em `Send Test Payload`, enviaremos dois eventos de teste para verificar se você valida a chave de API.

Depois que validar, clique em `Save`.

### Etapa 4: Adicionar um comentário

Agora você pode adicionar, editar ou excluir comentários e deverá ver que chamamos sua máquina de desenvolvimento local com os eventos, usando sua chave de API de teste. Pode haver até 30 segundos de atraso para que os eventos cheguem à sua máquina.