### Maven

将 Repsy 仓库添加到项目的 POM 文件：

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

然后添加你需要的依赖：

```xml
<dependencies>
    <!-- API 客户端 -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- 核心库（包含 SSO） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub 库（用于实时事件） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
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
    implementation "com.fastcomments:client:1.3.1"
    
    // 核心库（包含 SSO）
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub 库（用于实时事件）
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### 库内容

此库包含三个模块。生成的 API 客户端、包含手写实用工具以简化与 API 交互的核心 Java 库，以及用于订阅更改推送的 `pubsub` 模块。

- [API 客户端库文档](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [核心库文档，包括 SSO 示例](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 库文档](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公共与受保护的 API

对于 API 客户端，有两个类，`DefaultApi` 和 `PublicApi`。`DefaultApi` 包含需要你的 API 密钥的方法，而 `PublicApi` 包含可以直接从浏览器/移动设备等在无身份验证情况下调用的 API 调用。