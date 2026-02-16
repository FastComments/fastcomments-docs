1.  **Se i aktion.**

    Gå ind i mappen for dit nye site, og start det op.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Tilføj FastComments til dit Gatsby-site.**

    Se på src/pages/index.js. Denne ene linje viser, hvordan vi opretter widgeten:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```