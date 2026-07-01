### Maven

添加 Repsy 仓库到项目的 POM 中：

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
    <!-- API 客户端 -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- 核心库（包含 SSO） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub 库（用于实时事件） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

添加 Repsy 仓库到您的 `build.gradle` 文件中：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API 客户端
    implementation "com.fastcomments:client:3.0.0"
    
    // 核心库（包含 SSO）
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub 库（用于实时事件）
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Library Contents

该库包含三个模块。生成的 API 客户端、包含手写工具函数以简化 API 使用的核心 Java 库，以及用于订阅变更流的 `pubsub` 模块。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

对于 API 客户端，有三个类：`DefaultApi`、`PublicApi` 和 `ModerationApi`。`DefaultApi` 包含需要 API 密钥的方法，`PublicApi` 包含可以直接从浏览器/移动设备等无需身份验证调用的方法。

`ModerationApi` 提供了大量实时且快速的审核 API。每个 `ModerationApi` 方法都接受 `sso` 参数，并且可以通过 SSO 或 FastComments.com 会话 cookie 进行身份验证。