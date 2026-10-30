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

然后添加您需要的依赖：

```xml
<dependencies>
    <!-- API 客户端 -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- 核心库（包括 SSO） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub 库（用于实时事件） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

将 Repsy 仓库添加到您的 build.gradle 文件中：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API 客户端
    implementation "com.fastcomments:client:3.1.1"
    
    // 核心库（包括 SSO）
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub 库（用于实时事件）
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Library Contents

此库包含三个模块：生成的 API 客户端、包含手写实用工具以简化 API 使用的核心 Java 库，以及 `pubsub` 模块（用于订阅变更推送的库）。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [核心库文档（包括 SSO 示例）](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 库文档](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

对于 API 客户端，有三个类，`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要您的 API 密钥的方法，`PublicApi` 包含可以直接从浏览器/移动设备等发起且无需身份验证的方法。

`ModerationApi` 提供了丰富的实时和快速审核 API。每个 `ModerationApi` 方法都接受一个 `sso` 参数，并且可以通过 SSO 或 FastComments.com 会话 cookie 进行身份验证。