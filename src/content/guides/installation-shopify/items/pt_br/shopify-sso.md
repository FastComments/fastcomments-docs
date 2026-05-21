O bloco **FastComments** oferece single sign-on para que seus clientes do Shopify possam comentar como eles mesmos sem criar uma conta separada no FastComments.

### Como funciona

Quando um visitante que está logado na sua loja abre uma página com o bloco **FastComments**:

1. O bloco detecta o objeto Shopify `customer`.
2. Ele envia o nome e o e-mail do cliente para o FastComments através de uma requisição proxy do aplicativo assinada.
3. O FastComments cria ou associa um usuário identificado por `shopify-{customerId}`, assim o mesmo cliente sempre corresponde ao mesmo usuário do FastComments entre sessões e reinstalações.
4. O nome do visitante aparece em seus comentários. Eles não precisam fazer login novamente.

Se o visitante não estiver logado na loja, o bloco volta para comentários anônimos (ou para o fluxo de login do FastComments, dependendo da configuração do seu widget).

### Desativando o SSO

O SSO está ativado por padrão em cada bloco **FastComments**. Para desativá-lo em um bloco específico:

1. Abra o editor de tema do Shopify.
2. Abra o template que contém o bloco e clique no bloco para selecioná-lo.
3. Desmarque **SSO**.
4. Clique em **Salvar**.

Desative o SSO se você quiser que os comentaristas escolham uma identidade separada para a conversa. Por exemplo, uma página de comunidade interna onde a equipe comenta com um nome de exibição diferente.

### O que o FastComments recebe

A carga útil SSO enviada para cada cliente contém:

- Um ID de usuário derivado do ID do cliente do Shopify (`shopify-{customerId}`).
- O e-mail do cliente (usado para identificar o usuário; não é exibido publicamente).
- O nome de exibição do cliente (usado como nome do autor do comentário).

Nenhum histórico de pedidos, pagamento ou dados de endereço é enviado. A carga útil é assinada no servidor; o navegador do cliente nunca vê uma credencial.

### Links de login e logout

Quando o SSO está ativado, os links de entrar e sair do widget de comentários apontam para `/account/login` e `/account/logout`, as rotas padrão de conta de cliente do Shopify. Não há nada a configurar. Os links funcionam para qualquer loja com contas de cliente habilitadas.