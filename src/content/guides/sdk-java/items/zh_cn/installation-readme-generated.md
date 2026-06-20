### Maven

将 Repsy 仓库添加到项目的 POM 中：

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

然后添加所需的依赖项：

```xml
<dependencies>
    <!-- API 客户端 -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- 核心库（包含 SSO） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub 库（用于实时事件） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

将 Repsy 仓库添加到你的 build.gradle 文件：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API 客户端
    implementation "com.fastcomments:client:2.0.0"
    
    // 核心库（包含 SSO）
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub 库（用于实时事件）
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

此库包含三个模块。生成的 API 客户端、包含手写工具以简化与 API 交互的核心 Java 库，以及用于订阅变更流的 `pubsub` 模块库。

- [API 客户端库 文档](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [核心库文档，包含 SSO 示例](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 库 文档](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

对于 API 客户端，有三个类：`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要你的 API 密钥 的方法，`PublicApi` 包含可以直接从浏览器/移动设备/等无认证情况下调用的方法。

`ModerationApi` 为版主控制面板提供支持。它包含评论审核的方法（列出、计数、搜索、日志和导出）、审核操作（删除/恢复、标记、设置审核/垃圾/通过 状态、投票，以及重新打开/关闭主题）、封禁（从评论中封禁、撤销封禁、封禁前摘要、封禁状态和偏好，以及被封用户计数）以及徽章与信任（授予/移除徽章、手动徽章、获取/设置信任因子和用户内部资料）。每个 `ModerationApi` 方法都接受一个 `sso` 参数，以便代表已通过 SSO 验证的版主执行调用。