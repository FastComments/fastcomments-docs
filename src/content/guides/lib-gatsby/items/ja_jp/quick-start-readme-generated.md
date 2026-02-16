1.  **実際に動作を確認する。**

    新しいサイトのディレクトリに移動して起動します。

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **Gatsby サイトに FastComments を追加する。**

    src/pages/index.js を確認してください。この1行がウィジェットをインスタンス化する方法です：

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```