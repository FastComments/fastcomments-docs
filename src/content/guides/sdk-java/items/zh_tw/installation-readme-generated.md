### Maven

將 Repsy 儲存庫新增到您專案的 POM 檔案：

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Then add the dependencies you need:

```xml
<dependencies>
    <!-- API 用戶端 -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- 核心函式庫（包含 SSO） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub 函式庫（用於即時事件） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
    </dependency>
</dependencies>
```

### Gradle

將 Repsy 儲存庫新增到您的 build.gradle 檔案：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API 用戶端
    implementation "com.fastcomments:client:1.3.1"
    
    // 核心函式庫（包含 SSO）
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub 函式庫（用於即時事件）
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Library Contents

This library contains three modules. The generated API client, the core Java library which contains hand-written utilities
to make working with the API easier, and the `pubsub` module which is a library for subscribing to change feeds.

- [API 用戶端函式庫文件](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [核心函式庫文件，包含 SSO 範例](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub 函式庫文件](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公開與受保護的 API

對於 API 用戶端，有兩個類別，`DefaultApi` 和 `PublicApi`。`DefaultApi` 包含需要您的 API 金鑰的方法，而 `PublicApi` 包含可以直接從瀏覽器／行動裝置等在未經驗證的情況下呼叫的 API。