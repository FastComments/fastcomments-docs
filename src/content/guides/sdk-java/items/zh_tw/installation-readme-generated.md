### Maven

在您專案的 POM 中加入 Repsy 儲存庫：

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

然後加入您需要的依賴：

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

在您的 build.gradle 檔案中加入 Repsy 儲存庫：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### 函式庫內容

此函式庫包含三個模組：自動產生的 API 用戶端、包含手寫工具程式以簡化 API 操作的核心 Java 函式庫，以及用於訂閱變更串流的 `pubsub` 模組。

- [API Client Library 文件](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library 文件，包含 SSO 範例](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library 文件](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公開 API 與受保護 API

API 用戶端包含三個類別：`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要您的 API 金鑰的方法，而 `PublicApi` 包含可直接從瀏覽器/行動裝置等進行呼叫、無需驗證的方法。

`ModerationApi` 提供一套廣泛且即時快速的審核 API。每個 `ModerationApi` 方法都接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的 session cookie 進行驗證。