Uma vez que um administrador registrou o FastComments como uma ferramenta LTI 1.3 Advantage e aprovou as políticas da instituição, os instrutores o adicionam aos cursos através dos pontos de posicionamento padrão do Blackboard. As etapas exatas diferem entre Ultra Course View e Original Course View, então ambas são abordadas abaixo.

#### Ultra Course View

Ultra Course View é o padrão no Blackboard Learn SaaS a partir de 2026.

1. Abra o curso e vá para a página **Course Content**.
2. Passe o cursor ou toque no local onde você quer que o tópico de comentários apareça na estrutura e clique no botão roxo **+** (Add content).
3. Escolha **Content Market**. O painel Content Market lista todas as ferramentas LTI aprovadas e colocações de Building Block da sua instituição.
4. Encontre o bloco **FastComments** e clique nele. O Blackboard cria um item de conteúdo na posição onde você abriu o menu **+**.
5. O item aparece na estrutura como uma entrada "Visible to students" por padrão para instrutores que têm **Hide from students** desativado como padrão pessoal. Se o seu padrão for **Hidden**, o item é criado oculto e você ativa o seletor de visibilidade na linha do item quando estiver pronto.
6. Para renomear o item, clique no título na estrutura e digite um novo rótulo. O título que os alunos veem na estrutura é independente do identificador do thread do FastComments, portanto renomear é seguro a qualquer momento.

Se você não vir **Content Market** como opção, sua instituição ocultou a colocação. Você também acessa o mesmo seletor através de **More tools** no mesmo menu **+** sob o grupo **LTI Tools**.

#### Original Course View

Original Course View ainda é suportado no Learn SaaS e permanece a experiência principal para sites Learn 9.1 self-hosted na linha de release CU do Q4 2024.

1. Abra o curso e entre em uma **Content Area** (por exemplo, a **Information** ou **Content** padrão no menu do curso).
2. Ative o **Edit Mode** com o interruptor no canto superior direito da página.
3. Clique em **Build Content** na barra de ações.
4. No submenu **Learning Tools**, clique em **FastComments**. O submenu Learning Tools é preenchido a partir das colocações de ferramentas LTI 1.3 depois que um administrador registra a ferramenta. Se você não o vir, veja a seção de problemas comuns abaixo.
5. No formulário **Create FastComments**, configure:
   - **Name**: o rótulo que os alunos veem na área de conteúdo.
   - **Description**: texto opcional exibido acima do thread incorporado.
   - **Permit Users to View this Content**: alternador de disponibilidade Yes/No.
   - **Track Number of Views**: habilite se quiser as estatísticas de visualizações por item do Blackboard. FastComments executa suas próprias análises independentemente.
   - **Date and Time Restrictions**: janelas opcionais **Display After** / **Display Until**.
6. Envie. A ferramenta aparece como um item clicável na área de conteúdo.

#### Embedding Inside an Item or Document

Em ambas as visualizações de curso, os instrutores incorporam o FastComments inline dentro do corpo de um Item, Documento ou qualquer campo rich-text através do botão LTI Advantage do Content Editor.

Ultra Course View:

1. Crie ou edite um **Document**.
2. Clique em **Add content** dentro do corpo do documento onde deseja que o thread apareça.
3. Na barra de ferramentas do editor, abra o menu **Insert content** e clique em **Content Market** (o ponto de entrada LTI Advantage / Deep Linking).
4. Escolha **FastComments**. O FastComments retorna um payload de deep-link e o Blackboard insere um bloco incorporado no corpo do documento na posição do cursor.
5. Salve o documento. Os alunos veem o thread renderizado inline enquanto rolam a página.

Original Course View:

1. Edite qualquer item com um corpo rich-text.
2. Na barra de ferramentas do Content Editor, clique no ícone de mais **Add Content** e escolha **Content Market** (rotulado **Add Content from External Tool** em CUs mais antigos do Q4 2024).
3. Escolha **FastComments**. O editor insere um bloco de espaço reservado referenciando o recurso deep-linked.
4. Envie o item.

Cada embed de deep-link produz seu próprio thread FastComments, então um Item com dois blocos FastComments incorporados tem duas transmissões de comentários independentes.

#### Visibility, Release Conditions, and Group Restrictions

Os itens de conteúdo do FastComments se comportam como qualquer outro item de conteúdo do Blackboard para as regras de controle de acesso aplicadas sobre eles.

- Ultra: clique no seletor de visibilidade na linha (**Visible to students**, **Hidden from students**, **Conditional availability**). A disponibilidade condicional suporta janelas de data/hora, regras de desempenho contra itens do gradebook e regras de membro contra grupos do curso.
- Original: abra o menu de contexto do item e escolha **Adaptive Release** ou **Adaptive Release: Advanced** para bloquear a ferramenta por data, filiação, nota ou status de revisão. Use **Set Group Availability** no item para restringir a grupos específicos do curso.

O FastComments respeita o que quer que o gate do Blackboard decida. Se o Blackboard ocultar o item de um aluno, o launch LTI nunca ocorre para esse aluno e ele não aparece na visualização de moderador.

#### Gradebook Behavior

O FastComments não reporta notas de volta via LTI Advantage Assignment and Grade Services. Nenhuma coluna de nota é criada automaticamente para itens de conteúdo do FastComments.

Se o seu tenant Blackboard estiver configurado para criar automaticamente uma coluna de grade para todo novo item de conteúdo independentemente dos metadados de avaliação, uma coluna vazia aparece mesmo assim. Para ocultá-la:

- Ultra: abra o **Gradebook**, clique no cabeçalho da coluna, escolha **Edit** e desative **Show to students** além de **Include in calculations**. Ou use **Delete** se sua instituição permitir a exclusão de colunas para itens sem avaliação.
- Original: abra o **Grade Center**, clique na chevron da coluna, escolha **Hide from Users (on/off)** e, opcionalmente, **Hide from Instructor View** em **Column Organization**.

#### What Students See

Quando um aluno abre o item FastComments ou rola até um bloco incorporado:

1. O Blackboard lança a mensagem LTI 1.3 para o FastComments. O aluno é autenticado via SSO usando sua identidade do Blackboard (nome, email, avatar, função) sem ver um formulário de login.
2. O thread de comentários é renderizado no iframe. Encadeamento, respostas, menções e reações estão todas disponíveis com base nas configurações do widget de comentários configuradas no FastComments.
3. Seus comentários são atribuídos à conta do Blackboard. Se o aluno editar seu nome ou foto no Blackboard posteriormente, o próximo lançamento atualiza o perfil do FastComments.

Mapeamento de funções do Blackboard para o FastComments:

- **System Administrator** e **Course Builder** mapeiam para **admin** do FastComments.
- **Instructor** e **Teaching Assistant** mapeiam para **moderator** do FastComments.
- **Student**, **Guest**, e **Observer** mapeiam para **commenter** do FastComments.

Moderadores veem controles de moderação (pin, hide, ban, delete) inline em cada comentário do thread.

#### Thread Scoping

O FastComments delimita cada thread por **(Blackboard host, course ID, resource link ID)**. Dois itens FastComments no mesmo curso produzem dois threads. O mesmo item copiado em dois shells de curso (por exemplo, através de cópia de curso) produz dois threads, porque o Blackboard emite um novo resource link ID durante a cópia. Para manter um thread compartilhado entre cópias de curso, use Deep Linking com um URN de thread explícito configurado no FastComments antes de iniciar a cópia.

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** O administrador aprovou a ferramenta mas deixou uma política da instituição bloqueando a colocação relevante. Vá para **Administrator Panel** > **Integrations** > **LTI Tool Providers**, edite a entrada do FastComments e confirme que tanto **Course Content Tool** (Original) quanto **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) placements estão habilitados. Salve e atualize a página do curso.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** O escopo de deployment registrado durante o registro dinâmico não corresponde ao contexto da instituição ao qual o curso pertence. Na entrada do provedor de ferramentas do Blackboard, verifique se o **Deployment ID** corresponde ao que o FastComments mostra na sua página LTI 1.3 Configuration para este tenant. Se diferirem, exclua a colocação e reexecute o registro dinâmico a partir de uma URL de registro nova (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha-o aqui</a>).

**Iframe height looks fixed or content gets cut off.** Alguns tenants do Blackboard vêm com uma Content Security Policy estrita que bloqueia o postMessage padrão de redimensionamento de iframe LTI. O FastComments emite tanto a mensagem no estilo Canvas `lti.frameResize` quanto a mensagem no formato especificação IMS `org.imsglobal.lti.frameResize` para maximizar a compatibilidade, mas uma sobrescrita de CSP em nível de tenant bloqueia o listener pai. Peça ao seu administrador para confirmar que `*.fastcomments.com` está na allowlist de ferramentas LTI e que nenhum cabeçalho CSP personalizado está removendo eventos postMessage. O redimensionamento então funciona sem configuração adicional.

**Course copy duplicates threads.** A cópia de curso do Blackboard emite novos resource link IDs para colocações LTI, então cursos copiados começam com threads vazios. Isso é esperado. Se você precisar que o curso copiado herde o thread original, configure Deep Linking com um URN de thread explícito antes de copiar, ou contate o suporte do FastComments para remapear IDs de thread em massa.

**Student sees a generic Blackboard error on launch.** A causa é uma claim `email` ausente ou desatualizada. Confirme a política da instituição para o FastComments tem **Role**, **Name**, e **Email Address** habilitados em **User Fields to Send**. Salve e então inicie novamente em uma sessão de navegador nova.