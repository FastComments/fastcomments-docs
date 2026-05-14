**Usando o Moodle?** Também publicamos um plugin dedicado para Moodle do FastComments com uma integração mais profunda do que o LTI 1.3 (hooks de sincronização de notas, relatórios de atividade mais detalhados, interface nativa de configurações do Moodle). Veja o <a href="/guide-installation-moodle.html" target="_blank">guia de instalação do plugin para Moodle</a>. O fluxo LTI 1.3 abaixo é a escolha certa se você quiser um único registro que também cubra outros LMSs, ou se o administrador do seu Moodle não instalar plugins de terceiros.

O Moodle 4.0+ oferece suporte ao registro dinâmico LTI 1.3 através do plugin External tool.

#### Abra a tela de Gerenciamento de Ferramentas

1. Faça login no Moodle como administrador do site.
2. Navegue até **Administração do site** > **Plugins** > **Módulos de atividade** > **Ferramenta externa** > **Gerenciar ferramentas**.

#### Cole a URL

Você verá um cartão rotulado **Tool URL**. Cole a URL de registro do FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenha-o aqui</a>) no campo de texto e clique em **Add LTI Advantage**.

O Moodle abre uma tela de registro mostrando a identidade da ferramenta e as permissões que ela está solicitando. Revise e clique em **Ativar** (ou **Registrar**, dependendo da versão do Moodle).

A janela pop-up se fecha quando o registro é concluído; a nova ferramenta FastComments aparece na lista **Tools** com o status **Active**.

#### Tornar disponível

Por padrão, o Moodle adiciona novas ferramentas à lista "Course tools" mas não as mostra no seletor de atividades. Para expor o FastComments em todo o curso:

1. Clique no ícone de engrenagem no bloco do FastComments.
2. Em **Tool configuration usage**, escolha **Show in activity chooser and as a preconfigured tool**.
3. Salve.

Os instrutores agora podem adicionar o FastComments a qualquer curso através de **Adicionar uma atividade ou recurso** > **FastComments**.