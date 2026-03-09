#### Abrir Chaves de Desenvolvedor no Canvas

Faça login no Canvas como administrador. Vá para **Administrador** (na barra lateral esquerda) > selecione sua conta > **Chaves de Desenvolvedor**.

#### Criar uma Chave de Desenvolvedor LTI

Clique em **+ Chave de Desenvolvedor** e selecione **Chave LTI**.

No formulário de configuração:

1. No campo **URIs de Redirecionamento** (lado esquerdo), cole a **Launch URL** da página de configuração do FastComments.
2. À direita, defina **Método** como **Inserir URL**.
3. Cole a **Configuration URL** que você copiou do FastComments no campo **JSON URL**.
4. O Canvas carregará a configuração LTI automaticamente.
5. Dê um nome à chave (por exemplo, "FastComments").
6. Clique em **Salvar**.

#### Ativar a Chave de Desenvolvedor

Após salvar, a nova chave aparecerá na tabela de Chaves de Desenvolvedor com seu **Estado** definido como **DESATIVADO**. Clique no interruptor para defini-lo como **ATIVADO**. O Canvas pode solicitar que você confirme — clique em **Permitir** para habilitar a chave.

#### Copiar o Client ID

A tabela de Chaves de Desenvolvedor mostra um **Client ID** numérico na coluna Detalhes (por exemplo, `17000000000042`). Copie esse número - você precisará dele na próxima etapa.