1.  **Pogledajte u akciji.**

    Uđite u direktorij svog novog sajta i pokrenite ga.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Dodajte FastComments na svoj Gatsby sajt.**

    Pogledajte datoteku src/pages/index.js. Ovaj jedan redak pokazuje kako instanciramo widget:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```