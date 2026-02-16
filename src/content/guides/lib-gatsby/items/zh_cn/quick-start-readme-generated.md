---
1.  **查看演示。**

    进入你新站点的目录并启动它。

    ```shell
    cd fastcomments-gatsbyjs-example/
    npm install
    gatsby develop
    ```

2.  **将 FastComments 添加到你的 Gatsby 站点。**

    查看 src/pages/index.js。下面这一行展示了我们如何实例化该小部件：

    ```
    return <FastCommentsCommentWidget tenantId="demo" />;
    ```
---