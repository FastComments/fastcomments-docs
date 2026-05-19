Uma vez que um administrador registrou o FastComments como uma ferramenta LTI 1.3 Advantage e aprovou as políticas da instituição, os instrutores o adicionam aos cursos através dos pontos de inclusão padrão do Blackboard. Os passos exatos diferem entre Ultra Course View e Original Course View, então ambos são cobertos abaixo.

#### Visualização de Curso Ultra

Ultra Course View é o padrão no Blackboard Learn SaaS a partir de 2026.

1. Abra o curso e vá para a página **Course Content**.
2. Passe o mouse ou toque onde você quer que o tópico de comentários apareça no sumário e clique no botão roxo **+** (Adicionar conteúdo).
3. Escolha **Content Market**. O painel Content Market lista todas as ferramentas LTI aprovadas e colocações de Building Block para sua instituição.
4. Encontre o bloco **FastComments** e clique nele. O Blackboard cria um item de conteúdo na posição onde você abriu o menu **+**.
5. O item aparece no sumário como uma entrada "Visible to students" por padrão para instrutores que têm **Hide from students** desativado como padrão pessoal. Se seu padrão for **Hidden**, o item é criado oculto e você ativa o seletor de visibilidade na linha do item quando estiver pronto.
6. Para renomear o item, clique no título no sumário e digite um novo rótulo. O título que os alunos veem no sumário é independente do identificador do tópico do FastComments, então renomear é seguro a qualquer momento.

Se você não vê **Content Market** como uma opção, sua instituição deixou a colocação oculta. Você também acessa o mesmo seletor através de **More tools** no mesmo menu **+** sob o grupo **LTI Tools**.

#### Visualização de Curso Original

Original Course View ainda é suportado no Learn SaaS e continua sendo a experiência principal para sites Learn 9.1 auto-hospedados na linha de release Q4 2024 CU.

1. Abra o curso e entre em uma **Content Area** (por exemplo, a **Information** ou **Content** padrão no menu do curso).
2. Ative o **Edit Mode** com o interruptor no canto superior direito da página.
3. Clique em **Build Content** na barra de ações.
4. No submenu **Learning Tools**, clique em **FastComments**. O submenu Learning Tools é preenchido a partir das colocações das ferramentas LTI 1.3 depois que um administrador registra a ferramenta. Se você não o vir, veja a seção de problemas comuns abaixo.
5. No formulário **Create FastComments**, defina:
   - **Name**: o rótulo que os alunos veem na área de conteúdo.
   - **Description**: texto opcional mostrado acima do tópico incorporado.
   - **Permit Users to View this Content**: alternador de disponibilidade Sim/Não.
   - **Track Number of Views**: habilite se quiser as estatísticas por item do Blackboard. O FastComments mantém sua própria análise independentemente.
   - **Date and Time Restrictions**: janelas opcionais **Display After** / **Display Until**.
6. Envie. A ferramenta aparece como um item clicável na área de conteúdo.

#### Incorporação Dentro de um Item ou Documento

Em ambas as visualizações de curso, os instrutores incorporam o FastComments inline dentro do corpo de um Item, Documento ou qualquer campo rich-text através do botão LTI Advantage do Editor de Conteúdo.

Ultra Course View:

1. Crie ou edite um **Document**.
2. Clique em **Add content** dentro do corpo do documento onde você quer que o tópico apareça.
3. Na barra de ferramentas do editor, abra o menu **Insert content** e clique em **Content Market** (o ponto de entrada LTI Advantage / Deep Linking).
4. Escolha **FastComments**. O FastComments retorna uma payload de deep-link e o Blackboard insere um bloco incorporado no corpo do documento na posição do cursor.
5. Salve o documento. Os alunos veem o tópico renderizado inline conforme rolarem a página.

Original Course View:

1. Edite qualquer item com um corpo rich-text.
2. Na barra de ferramentas do Content Editor, clique no ícone de mais **Add Content** e escolha **Content Market** (rotulado **Add Content from External Tool** em CUs mais antigas de Q4 2024).
3. Escolha **FastComments**. O editor insere um bloco placeholder referenciando o recurso deep-linked.
4. Envie o item.

Cada incorporação deep-link produz seu próprio tópico FastComments, então um Item com dois blocos FastComments incorporados tem duas sequências de comentários independentes.

#### Visibilidade, Condições de Liberação e Restrições de Grupo

Os itens de conteúdo do FastComments se comportam como qualquer outro item de conteúdo do Blackboard quanto às regras de controle de acesso aplicadas sobre eles.

- Ultra: clique no seletor de visibilidade na linha (**Visible to students**, **Hidden from students**, **Conditional availability**). A disponibilidade condicional suporta janelas de data/hora, regras de desempenho contra itens do gradebook e regras de membro contra grupos do curso.
- Original: abra o menu de contexto do item e escolha **Adaptive Release** ou **Adaptive Release: Advanced** para restringir a ferramenta por data, associação, nota ou status de revisão. Use **Set Group Availability** no item para restringir a grupos específicos do curso.

O FastComments respeita o que quer que o mecanismo de controle do Blackboard decida. Se o Blackboard esconder o item de um aluno, o lançamento LTI nunca acontece para esse aluno e ele não aparece na visão do moderador.

#### Comportamento no Gradebook

O FastComments não reporta notas de volta via LTI Advantage Assignment and Grade Services. Nenhuma coluna de notas é criada automaticamente para itens de conteúdo do FastComments.

Se seu tenant do Blackboard estiver configurado para criar automaticamente uma coluna do gradebook para cada novo item de conteúdo independentemente dos metadados de avaliação, uma coluna vazia aparece mesmo assim. Para escondê-la:

- Ultra: abra o **Gradebook**, clique no cabeçalho da coluna, escolha **Edit** e desligue **Show to students** mais **Include in calculations**. Ou use **Delete** se sua instituição permitir a exclusão de colunas para itens sem avaliação.
- Original: abra o **Grade Center**, clique no chevron da coluna, escolha **Hide from Users (on/off)**, e opcionalmente **Hide from Instructor View** sob **Column Organization**.

#### O Que os Alunos Vêem

Quando um aluno abre o item do FastComments ou rola até um bloco incorporado:

1. O Blackboard lança a mensagem LTI 1.3 para o FastComments. O aluno é autenticado via SSO usando sua identidade do Blackboard (nome, email, avatar, função) sem ver um formulário de login.
2. O tópico de comentários é renderizado no iframe. Encadeamento, respostas, menções e reações estão todos disponíveis com base nas configurações do widget de comentários configuradas no FastComments.
3. Seus comentários são atribuídos à conta Blackboard deles. Se o aluno editar seu nome ou foto no Blackboard mais tarde, o próximo lançamento atualiza o perfil do FastComments.

Mapeamento de funções do Blackboard para o FastComments:

- **System Administrator** e **Course Builder** mapeiam para **admin** do FastComments.
- **Instructor** e **Teaching Assistant** mapeiam para **moderator** do FastComments.
- **Student**, **Guest**, e **Observer** mapeiam para **commenter** do FastComments.

Moderadores veem controles de moderação (fixar, ocultar, banir, excluir) inline em cada comentário no tópico.

#### Restringir o Acesso Público (Recomendado)

Por padrão, os dados de comentários do FastComments são legíveis publicamente. Qualquer pessoa que conseguir adivinhar a URL do tópico ou o endpoint da API pode ver seus comentários, mesmo fora do Blackboard. Para discussões de curso, você quase certamente vai querer restringir a visualização apenas aos alunos matriculados.

Abra sua <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">página de personalização do widget</a> e crie uma regra com **Require SSO To View Comments** habilitado, então defina o nível de segurança para **Secure SSO** para que os tópicos só possam ser carregados através do lançamento LTI assinado.

Veja [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) para o guia completo, incluindo como escopar a regra para um único domínio ou página.

#### Escopo do Tópico

O FastComments define o escopo de cada tópico por **(Blackboard host, course ID, resource link ID)**. Dois itens FastComments no mesmo curso produzem dois tópicos. O mesmo item copiado entre duas turmas (por exemplo, através de cópia de curso) produz dois tópicos, porque o Blackboard emite um novo resource link ID durante a cópia. Para manter um tópico compartilhado entre cópias de curso, use Deep Linking com um URN de tópico explícito configurado no FastComments antes de executar a cópia.

#### Problemas Específicos do Blackboard

**Bloco FastComments ausente do menu Build Content (Original) ou do Content Market (Ultra).** O administrador aprovou a ferramenta mas deixou uma política da instituição bloqueando a colocação relevante. Vá para **Administrator Panel** > **Integrations** > **LTI Tool Providers**, edite a entrada do FastComments e confirme que as colocações **Course Content Tool** (Original) e **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) estão habilitadas. Salve e atualize a página do curso.

**Erro "Tool not configured for this context" ou "Tool is not deployed" no lançamento.** O escopo de implantação registrado durante o registro dinâmico não corresponde ao contexto da instituição ao qual o curso pertence. Na entrada do provedor de ferramenta do Blackboard, verifique se o **Deployment ID** corresponde ao que o FastComments mostra na sua página de LTI 1.3 Configuration para este tenant. Se divergirem, exclua a colocação e reexecute o registro dinâmico a partir de uma URL de registro nova (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>).

**Altura do iframe parece fixa ou o conteúdo é cortado.** Alguns tenants do Blackboard são enviados com uma Content Security Policy restritiva que bloqueia o postMessage de redimensionamento de iframe LTI padrão. O FastComments emite tanto a mensagem no estilo Canvas `lti.frameResize` quanto a mensagem no formato da especificação IMS `org.imsglobal.lti.frameResize` para maximizar a compatibilidade, mas uma sobrescrição de CSP a nível de tenant bloqueia o listener no pai. Peça ao seu administrador para confirmar que `*.fastcomments.com` está na allowlist de ferramentas LTI e que nenhum header CSP customizado está removendo eventos postMessage. O redimensionamento então funciona sem configuração adicional.

**Cópia de curso duplica tópicos.** A cópia de curso do Blackboard emite novos resource link IDs para colocações LTI, então cursos copiados começam com tópicos vazios. Isso é esperado. Se você precisa que o curso copiado herde o tópico original, configure Deep Linking com um URN de tópico explícito antes de copiar, ou contate o suporte do FastComments para remapear IDs de tópicos em massa.

**O aluno vê um erro genérico do Blackboard no lançamento.** A causa é uma claim `email` ausente ou desatualizada. Confirme que a política da instituição para o FastComments tem **Role**, **Name**, e **Email Address** habilitados em **User Fields to Send**. Salve, então tente o lançamento novamente em uma nova sessão do navegador.