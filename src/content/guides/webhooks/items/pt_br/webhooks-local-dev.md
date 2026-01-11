---
Para desenvolvimento local, use uma ferramenta como [ngrok](https://ngrok.com/).

Para simplificar a manutenção da segurança do sistema, o desenvolvimento local segue o mesmo processo de configuração e proteção de outros ambientes. 

### Etapa 1: Adicione "localhost" aos domínios na sua conta.

Adicione "localhost" [como um domínio aqui](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Etapa 2: Escolha uma chave de API

Vamos adicionar a configuração do webhook para o seu domínio, então precisaremos de uma chave de API. [Você pode fazer isso aqui.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Em "Associate with domain" - selecione seu domínio "localhost".

**OBSERVAÇÃO: Alternativamente, você pode usar um API Secret para toda a atividade de teste e ambientes de staging. Simplesmente adicione um API Secret para "All Domains", e dê um nome como "test".**

Certifique-se de ter um API Secret definido para seus domínios de produção. Eventos para todos os outros domínios usarão o secret curinga (de teste).

### Etapa 3: Adicione seu Webhook

Enquanto estiver executando o ngrok ou ferramenta similar, defina o valor para "localhost" [aqui](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

Ao clicar em `Send Test Payload`, enviaremos dois eventos de teste para verificar se você valida a chave de API.

Uma vez validado, clique em `Save`.

### Etapa 4: Adicione um comentário

Agora você pode adicionar, editar ou excluir comentários e deverá nos ver chamar sua máquina de desenvolvimento local com os eventos, usando sua chave de API de teste. Pode haver um atraso de até 30 segundos
para os eventos chegarem à sua máquina.

---