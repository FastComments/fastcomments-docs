1.  **Pogledajte u akciji.**

    Uđite u direktorij vašeg novog sajta i pokrenite ga.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Dodajte FastComments na vaš Gatsby sajt.**

    Pogledajte src/pages/index.js. Ovaj jedan red pokazuje kako instanciramo widget:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```