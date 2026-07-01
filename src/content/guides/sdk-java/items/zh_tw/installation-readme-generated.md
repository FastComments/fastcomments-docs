### Maven

將 Repsy 儲存庫新增至您專案的 POM：

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

然後加入您需要的相依性：

```xml
<dependencies>
    <!-- API 客戶端 -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- 核心庫（包含 SSO） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub 庫（用於即時事件） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

將 Repsy 儲存庫新增至您的 `build.gradle` 檔案：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API 客戶端
    implementation "com.fastcomments:client:3.0.0"
    
    // 核心庫（包含 SSO）
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub 庫（用於即時事件）
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Library Contents

此庫包含三個模組：產生的 API 客戶端、包含手寫工具函式以簡化 API 使用的核心 Java 庫，以及 `pubsub` 模組——用於訂閱變更資訊的庫。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

對於 API 客戶端，提供三個類別：`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要您的 API 金鑰的功能，`PublicApi` 則提供可直接從瀏覽器／行動裝置等無需驗證即可呼叫的功能。

`ModerationApi` 提供廣泛的即時與快速審核 API。每個 `ModerationApi` 方法皆接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的會話 Cookie 進行驗證。