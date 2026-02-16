1.  **ראה בפעולה.**

    נווט אל תיקיית האתר החדש שלך והפעל אותה.

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **הוסף את FastComments לאתר ה-Gatsby שלך.**

    עיין ב- src/pages/index.js. שורה אחת זו היא כיצד אנו מאתחלים את הווידג'ט:

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```