D2L Brightspace expõe o Registro Dinâmico através da interface de administração LTI Advantage. Você precisará de acesso de administrador.

#### Abra a tela de registro

1. Faça login na sua instância do Brightspace como administrador.
2. Navegue até **Ferramentas do administrador** > **Gerenciar Extensibilidade** > **LTI Advantage**.
3. Clique em **Registrar ferramenta**. (A URL direta é `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Cole a URL

Você verá um formulário de registro. O campo principal é **Tool initiation registration endpoint** (algumas versões do Brightspace o rotulam como "Tool Initiation Registration URL").

Cole a URL de registro do FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha-a aqui</a>) nesse campo. Deixe os outros campos em branco - eles são preenchidos automaticamente pelo FastComments durante o handshake de registro.

Clique em **Registrar**.

#### Aprove a ferramenta

O Brightspace abre um pop-up que se comunica com o FastComments, troca chaves e mostra uma tela de confirmação. O pop-up se fecha automaticamente quando o registro é concluído.

A nova ferramenta aparece na sua lista de ferramentas LTI Advantage. Por padrão o Brightspace marca novas ferramentas como **desativada** - altere o interruptor para **ativada** para que seus cursos possam usá-la.

#### Adicionar uma implantação

No Brightspace, as ferramentas LTI precisam de uma **implantação** antes de poderem ser usadas em cursos:

1. Abra a ferramenta FastComments recém-registrada.
2. Clique em **Ver implantações** > **Nova implantação**.
3. Dê um nome à implantação (por exemplo, "FastComments - All Courses"), escolha as unidades organizacionais nas quais ela deve estar disponível e salve.

Depois do primeiro lançamento por meio desta implantação, o FastComments vincula o `deployment_id` ao seu registro de configuração - lançamentos subsequentes a partir de uma implantação diferente sob o mesmo cliente serão rejeitados, a menos que você registre novamente.