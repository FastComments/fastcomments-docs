Para desenvolvimento local, use uma ferramenta como [ngrok](https://ngrok.com/).

Para simplificar a manutenção da segurança do sistema, o desenvolvimento local segue o mesmo processo de configuração e proteção que outros ambientes. 

### Etapa 1: Adicione "localhost" aos domínios na sua conta.

Adicione "localhost" [como um domínio aqui](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Etapa 2: Escolha uma Chave de API

Vamos adicionar a configuração de webhook para o seu domínio, então precisaremos de uma chave de API. [Você pode fazer isso aqui.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Em "Associate with domain" - selecione seu domínio "localhost".

**NOTA: Alternativamente, você pode usar um API Secret para toda a atividade de teste e ambientes de staging. Basta adicionar um API Secret para "All Domains", e dar um nome como "test".**

Garanta que você tenha um API Secret definido para seu(s) domínio(s) de produção. Eventos de todos os outros domínios usarão o secret curinga (de teste).

### Etapa 3: Adicione seu Webhook

Enquanto estiver executando ngrok ou uma ferramenta similar, defina o valor para "localhost" [aqui](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Quando clicar em `Send Test Payload`, enviaremos dois eventos de teste para verificar se você valida a chave de API.

Uma vez validado, clique em `Save`.

### Etapa 4: Adicione Um Comentário

Agora você pode adicionar, editar ou excluir comentários e deve ver que chamamos sua máquina de desenvolvimento local com os eventos, usando sua chave de API de teste. Pode haver até 30 segundos de atraso
para os eventos chegarem à sua máquina.