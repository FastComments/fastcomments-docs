D2L Brightspace expõe o Registro Dinâmico através da interface de administração LTI Advantage. Você precisará de acesso de administrador.

#### Abra a tela de registro

1. Faça login na sua instância do Brightspace como administrador.
2. Navegue até **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Clique em **Register Tool**. (A URL direta é `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Cole a URL

Você verá um formulário de registro. O campo principal é **Tool initiation registration endpoint** (algumas versões do Brightspace o rotulam como "Tool Initiation Registration URL").

Cole a URL de registro do FastComments nesse campo. Deixe os outros campos em branco — eles são preenchidos automaticamente pelo FastComments durante o handshake de registro.

Clique em **Register**.

#### Aprove a ferramenta

O Brightspace abre um popup que se comunica com o FastComments, troca chaves e exibe uma tela de confirmação. O popup se fecha automaticamente quando o registro é concluído.

A nova ferramenta aparece na sua lista de ferramentas LTI Advantage. Por padrão o Brightspace marca novas ferramentas como **disabled** — ative o toggle para **enabled** para que seus cursos possam usá-la.

#### Adicione uma implantação

No Brightspace, ferramentas LTI precisam de uma **deployment** antes de poderem ser usadas em cursos:

1. Abra a ferramenta FastComments recém-registrada.
2. Clique em **View Deployments** > **New Deployment**.
3. Dê um nome à deployment (por exemplo, "FastComments - All Courses"), escolha as unidades organizacionais em que ela deve estar disponível e salve.

Após o primeiro lançamento através desta deployment, o FastComments fixa o `deployment_id` no seu registro de configuração — lançamentos subsequentes a partir de uma deployment diferente sob o mesmo cliente serão rejeitados a menos que você registre novamente.