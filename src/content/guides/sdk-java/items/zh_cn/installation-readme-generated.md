### Maven

将 Repsy 仓库添加到您项目的 POM 中：

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

然后添加您需要的依赖项：

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    // API Client
    implementation "com.fastcomments:client:1.3.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Library Contents

该库包含三个模块。生成的 API 客户端、包含手写实用程序以简化与 API 交互的核心 Java 库，以及用于订阅变更流的 `pubsub` 模块。

- [API Client 库文档](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core 库文档，包含 SSO 示例](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 库文档](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

对于 API 客户端，有两个类，`DefaultApi` 和 `PublicApi`。`DefaultApi` 包含需要您的 API 密钥的方法，而 `PublicApi` 包含可以直接从浏览器/移动设备等在不进行身份验证的情况下调用的 API。