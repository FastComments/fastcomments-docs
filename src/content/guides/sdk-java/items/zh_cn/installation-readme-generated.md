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

然后添加所需的依赖：

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

将 Repsy 仓库添加到 `build.gradle` 文件中：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### 库内容

此库包含三个模块：生成的 API 客户端、包含手写工具类以简化 API 使用的核心 Java 库，以及 `pubsub` 模块（用于订阅变更推送的库）。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [核心库文档（含 SSO 示例）](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 库文档](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公共 API 与受保护 API

对于 API 客户端，有三个类：`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要 API 密钥的方法，`PublicApi` 包含可以直接从浏览器、移动设备等无需身份验证调用的方法。

`ModerationApi` 提供了一个全面的实时快速审核 API 套件。每个 `ModerationApi` 方法都接受 `sso` 参数，并可通过 SSO 或 FastComments.com 会话 Cookie 进行身份验证。