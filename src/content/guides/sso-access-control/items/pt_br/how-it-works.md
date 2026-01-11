FastComments Controle de Acesso funciona atribuindo `Pages` e `Users` aos grupos desejados.

Um grupo é simplesmente um identificador de string, como `GREEN` ou `abc-123`.

`Users` e `Pages` não estão limitados a um único grupo. Eles são limitados a `100` e `1000` grupos, respectivamente. 

#### Acesso a Páginas Não Autorizadas

Se um usuário tentar acessar uma página à qual não tem acesso, verá uma mensagem de erro, como abaixo:

<div class="screenshot white-bg">
    <div class="title">Exemplo de Falha de Autorização</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Authorization Failure Example" />
</div>

O texto da mensagem pode ser personalizado.

---