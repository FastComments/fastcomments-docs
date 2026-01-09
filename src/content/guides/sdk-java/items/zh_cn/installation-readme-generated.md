### Maven

将 Repsy 仓库添加到您项目的 POM：

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
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
    </dependency>
</dependencies>
```

### Gradle

将 Repsy 仓库添加到您的 build.gradle 文件：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:0.0.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### 库内容

该库包含三个模块：生成的 API 客户端、包含手工编写实用工具以简化与 API 交互的核心 Java 库，以及用于订阅变更推送的 `pubsub` 模块。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [核心库文档（含 SSO 示例）](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 库文档](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公共 API 与 受保护 API

对于 API 客户端，有两个类：`DefaultApi` 和 `PublicApi`。`DefaultApi` 包含需要使用您的 API 密钥的方法，而 `PublicApi` 包含可直接从浏览器/移动设备等在无身份验证情况下调用的 API。