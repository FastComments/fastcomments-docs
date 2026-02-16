---
1.  **Vedi in azione.**

    Vai nella directory del tuo nuovo sito e avvialo.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Aggiungi FastComments al tuo sito Gatsby.**

    Dai un'occhiata a src/pages/index.js. Questa singola riga è come istanziamo il widget:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```
---