1.  **In Aktion ansehen.**

    Wechseln Sie in das Verzeichnis Ihrer neuen Website und starten Sie sie.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Fügen Sie FastComments zu Ihrer Gatsby-Website hinzu.**

    Schauen Sie sich src/pages/index.js an. Diese eine Zeile zeigt, wie wir das Widget instanziieren:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```