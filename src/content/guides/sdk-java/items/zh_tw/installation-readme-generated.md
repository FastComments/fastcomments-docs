### Maven

將 Repsy 儲存庫加入您專案的 POM：

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
        <version>3.1.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

將 Repsy 儲存庫加入您的 build.gradle 檔案：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### 函式庫內容

此函式庫包含三個模組：自動產生的 API client、包含手寫工具以簡化 API 操作的核心 Java 函式庫，以及用於訂閱變更串流的 `pubsub` 模組。

- [API Client 函式庫文件](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [核心函式庫文件（含 SSO 範例）](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 函式庫文件](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公開 API 與受保護 API

API client 提供三個類別：`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要您的 API 金鑰的方法，而 `PublicApi` 包含可直接從瀏覽器/行動裝置等進行呼叫、無需驗證的方法。

`ModerationApi` 提供一套廣泛且快速的即時審核 API。每個 `ModerationApi` 方法都接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的 session cookie 進行驗證。