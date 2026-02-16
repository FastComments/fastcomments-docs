1.  **See in Action.**

    Navigate into your new site’s directory and start it up.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Add FastComments to Your Gatsby Site.**

    Take a look at src/pages/index.js. This one line is how we instantiate the widget:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```