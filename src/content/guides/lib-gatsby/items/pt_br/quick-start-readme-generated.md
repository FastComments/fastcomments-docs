1.  **Veja em Ação.**

    Navegue até o diretório do seu novo site e inicie-o.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Adicione o FastComments ao seu site Gatsby.**

    Dê uma olhada em src/pages/index.js. Esta única linha é como instanciamos o widget:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```