1.  **查看實際效果。**

    切換到你新網站的目錄並啟動它。

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **將 FastComments 新增到你的 Gatsby 站點。**

    查看 src/pages/index.js。這一行就是我們實例化此元件的方式：

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```