1.  **Вижте в действие.**

    Влезте в директорията на новия си сайт и го стартирайте.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Добавете FastComments към своя Gatsby сайт.**

    Вижте src/pages/index.js. Този един ред показва как инстанцираме компонента:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```