1.  **Погледајте у акцији.**

    Пређите у директоријум вашег новог сајта и покрените га.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Додајте FastComments на ваш Gatsby сајт.**

    Погледајте src/pages/index.js. Ова једна линија показује како инстанцирамо видгет:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```